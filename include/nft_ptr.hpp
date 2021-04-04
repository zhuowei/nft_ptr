#pragma once
#include <cstdint>

extern "C" {
void WdbNftPtrInitialize(uint64_t owner_address, uint64_t caller_pc,
                         const char* ptr_object_type);
void WdbNftPtrMoveToken(uint64_t owner_address, uint64_t previous_owner_address,
                        uint64_t value, uint64_t caller_pc,
                        const char* object_type);
void WdbNftPtrDestroy(uint64_t owner_address);
}  // extern "C"

namespace wdb {
template <class T>
class nft_ptr final {
 public:
  explicit nft_ptr(T* ptr) : ptr_(ptr) {
    // TODO(zhuowei): __builtin_return_address(0) picks up the C1 constructor??
    void* caller_pc = __builtin_return_address(1);
    WdbNftPtrInitialize(reinterpret_cast<uint64_t>(this),
                        reinterpret_cast<uint64_t>(caller_pc),
                        typeid(T).name());
    SendMoveToken(this, /*previous_owner=*/nullptr, ptr, caller_pc);
  }

  template <class T2>
  nft_ptr(nft_ptr<T2>&& other) {
    void* caller_pc = __builtin_return_address(1);
    WdbNftPtrInitialize(reinterpret_cast<uint64_t>(this),
                        reinterpret_cast<uint64_t>(caller_pc),
                        typeid(T).name());
    T* new_ptr = other.ptr_;
    if (!new_ptr) {
      return;
    }
    other.ptr_ = nullptr;
    ptr_ = new_ptr;
    SendMoveToken(this, /*previous_owner=*/&other, new_ptr, caller_pc);
  }

  // used by make_nft
  nft_ptr(T* ptr, void* caller_pc) : ptr_(ptr) {
    WdbNftPtrInitialize(reinterpret_cast<uint64_t>(this),
                        reinterpret_cast<uint64_t>(caller_pc),
                        typeid(T).name());
    SendMoveToken(this, /*previous_owner=*/nullptr, ptr, caller_pc);
  }

  ~nft_ptr() {
    // D1 destructor, so need to do (1)??
    void* caller_pc = __builtin_return_address(1);
    if (ptr_) {
      SendMoveToken(/*owner=*/nullptr, this, ptr_, caller_pc);
      delete ptr_;
    }
    WdbNftPtrDestroy(reinterpret_cast<uint64_t>(this));
  }

  T* get() const { return ptr_; };

  T& operator*() const { return *ptr_; }

  T* operator->() const { return ptr_; }

  void reset(T* new_ptr = nullptr) {
    void* caller_pc = __builtin_return_address(0);
    T* old_ptr = ptr_;
    ptr_ = new_ptr;
    if (old_ptr) {
      SendMoveToken(/*owner=*/nullptr, this, old_ptr, caller_pc);
      delete old_ptr;
    }
    if (new_ptr) {
      SendMoveToken(this, /*previous_owner=*/nullptr, new_ptr, caller_pc);
    }
  }

  nft_ptr(const nft_ptr& Right) = delete;
  nft_ptr& operator=(const nft_ptr& Right) = delete;
  template <class T2>
  nft_ptr<T>& operator=(nft_ptr<T2>&& other) {
    void* caller_pc = __builtin_return_address(0);

    if (ptr_) {
      SendMoveToken(/*owner=*/nullptr, this, ptr_, caller_pc);
      delete ptr_;
      ptr_ = nullptr;
    }
    T* new_ptr = other.ptr_;
    if (!new_ptr) {
      return *this;
    }
    other.ptr_ = nullptr;
    ptr_ = new_ptr;
    SendMoveToken(this, /*previous_owner=*/&other, new_ptr, caller_pc);
    return *this;
  }

  // vars
  // TODO(zhuowei): make this a friend instead of public
  T* ptr_;

 private:
  static void SendMoveToken(void* owner, void* previous_owner, T* value,
                            void* caller_pc) {
    if (value) {
      WdbNftPtrMoveToken(reinterpret_cast<uint64_t>(owner),
                         reinterpret_cast<uint64_t>(previous_owner),
                         reinterpret_cast<uint64_t>(value),
                         reinterpret_cast<uint64_t>(caller_pc),
                         typeid(*value).name());
    }
  }
};

template <class T, class... Args>
nft_ptr<T> make_nft(Args&&... args) {
  return nft_ptr<T>(new T(args...), __builtin_return_address(0));
}

}  // namespace wdb
