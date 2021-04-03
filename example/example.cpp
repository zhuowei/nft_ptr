#include <iostream>
#include <vector>
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

class Zoo {
 public:
  std::vector<nft_ptr<Animal>> animals_;
  void AddAnimal(nft_ptr<Animal> animal);
  void MakeNoises() const;
};

void Zoo::AddAnimal(nft_ptr<Animal> animal) {
  animals_.push_back(std::move(animal));
}

void Zoo::MakeNoises() const {
  for (const auto& animal : animals_) {
    animal->MakeSound();
  }
}

int main(int argc, char** argv) {
  nft_ptr<Animal> ptr1(new Cow());
  ptr1->MakeSound();
  return 0;
}
