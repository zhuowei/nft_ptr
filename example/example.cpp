#include <iostream>
#include <vector>
#include "../include/nft_ptr.hpp"

using ::wdb::make_nft;
using ::wdb::nft_ptr;

class Animal {
 public:
  virtual ~Animal();
  virtual void MakeSound() = 0;
};

Animal::~Animal() {}

class Cow : public Animal {
 public:
  virtual void MakeSound() override;
};

void Cow::MakeSound() { std::cout << "Moo!" << std::endl; }

int main(int argc, char** argv) {
  std::cout << "Creating ptr1!" << std::endl;
  auto ptr1 = make_nft<Cow>();
  std::cout << "ptr1(" << &ptr1 << "): " << ptr1.get() << std::endl;
  ptr1->MakeSound();

  std::cout << "Creating ptr2!" << std::endl;
  nft_ptr<Animal> ptr2;
  std::cout << "ptr2(" << &ptr2 << "): " << ptr2.get() << std::endl;
  std::cout << "Moving: ptr2 = std::move(ptr1)" << std::endl;
  ptr2 = std::move(ptr1);
  std::cout << "Moved: ptr1 = " << ptr1.get() << " ptr2 = " << ptr2.get()
            << std::endl;

  std::cout << "Destroying objects" << std::endl;

  return 0;
}
