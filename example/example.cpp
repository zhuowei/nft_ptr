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

class Duck : public Animal {
 public:
  virtual void MakeSound() override;
};

void Duck::MakeSound() { std::cout << "Quack!" << std::endl; }

class Seal : public Animal {
 public:
  virtual void MakeSound() override;
};

void Seal::MakeSound() { std::cout << "Ow, ow, ow!" << std::endl; }

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
  std::cout << "Creating ptr1!" << std::endl;
  nft_ptr<Animal> ptr1(new Cow());
  std::cout << "\nptr1: " << ptr1.get() << std::endl;
  ;
  ptr1->MakeSound();
  std::cout << "\nCreating ptr2" << std::endl;
  auto ptr2 = make_nft<Duck>();
  std::cout << "\nptr2: " << ptr1.get() << std::endl;
  ;
  ptr2->MakeSound();
  std::cout << "Moving ptr1 = std::move(ptr2)" << std::endl;
  ptr1 = std::move(ptr2);
  std::cout << "\nptr1 after move: " << ptr1.get()
            << "\nptr2 after move: " << ptr2.get() << std::endl;

  std::cout << "\nCreating a zoo!" << std::endl;
  Zoo zoo;
  std::cout << "Adding animal from ptr1" << std::endl;
  zoo.AddAnimal(std::move(ptr1));
  std::cout << "\nAdding new animal" << std::endl;
  zoo.AddAnimal(make_nft<Seal>());
  std::cout << "\nMaking noises:" << std::endl;
  zoo.MakeNoises();
  std::cout << "\nDestroying everything" << std::endl;
  return 0;
}
