#include <iostream>
#include "../include/nft_ptr.hpp"

using ::wdb::nft_ptr;

class Animal {
 public:
  virtual ~Animal();
  virtual void MakeSound() = 0;
};

Animal::~Animal() {}

class Cow : public Animal {
  virtual void MakeSound() override;
};

void Cow::MakeSound() { std::cout << "Moo!" << std::endl; }

class Duck : public Animal {
  virtual void MakeSound() override;
};

void Duck::MakeSound() { std::cout << "Quack!" << std::endl; }

int main(int argc, char** argv) {
  nft_ptr<Animal> ptr1(new Cow());
  ptr1->MakeSound();
  return 0;
}
