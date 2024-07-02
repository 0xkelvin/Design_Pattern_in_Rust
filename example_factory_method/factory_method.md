The Factory Method pattern is a creational design pattern that provides an interface for creating objects in a superclass but allows subclasses to alter the type of objects that will be created. 

## Key Points
- Purpose: Define an interface for creating an object, but let subclasses decide which class to instantiate. Factory Method lets a class defer instantiation to subclasses.
- Use Case: When the exact type of objects created needs to be determined at runtime or subclasses need to specify the derived type of the jobject being created. 

## Structure
1. Product: Defines the interface of objects the factory method creates. 
2. ConcreteProduct: Implements the Product interface.
3. Creator: Declares the factory method which returns an object of type Product, May also define a default implementation of the factory method that returns a default ConcreteProduct object.
4. ConcreteCreator: Overrides the factory method to return an instance of a ConcreteProduct.
