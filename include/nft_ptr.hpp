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
class nft_ptr {
 public:
  explicit nft_ptr(T* ptr) : ptr_(ptr) {
    WdbNftPtrInitialize(reinterpret_cast<uint64_t>(this),
                        reinterpret_cast<uint64_t>(__builtin_return_address(0)),
                        typeid(T).name());
    if (ptr_) {
      WdbNftPtrMoveToken(
          reinterpret_cast<uint64_t>(this), 0, reinterpret_cast<uint64_t>(ptr_),
          reinterpret_cast<uint64_t>(__builtin_return_address(0)),
          typeid(*ptr_).name());
    }
  }

  ~nft_ptr() {
    if (ptr_) {
      delete ptr_;
    }
    WdbNftPtrDestroy(reinterpret_cast<uint64_t>(this));
  }

  T& operator*() const { return *ptr_; }

  T* operator->() const { return ptr_; }

  nft_ptr(const nft_ptr& Right) = delete;
  nft_ptr& operator=(const nft_ptr& Right) = delete;

 private:
  T* ptr_;
};
}  // namespace wdb
