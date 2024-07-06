# KeyPoint

## Purpose:
Provide an interface for creating families of related or dependent objects without specifying their concrete classes.

## Usecase:
When the creation process involves multiple related objects that should be used together.

## Structure:
1. AbstractFactory: Declares an interface for operations that create abstract product objects.
2. ConcreteFactory: Implement the operations to create concrete product objects
3. AbstractProduct: Declares an interface for a type of product objects
4. ConcreteProduct: Define a product object to be created by correspoinding concrete factory and implements the AbstractProduct interface
5. Client: Use only interface declared by AbstractFactory and AbstractProduct
