Design patterns are categorized into three main types: [Creational](https://github.com/0xkelvin/Design_Pattern_in_Rust/tree/main/Creational), Structural, and Behavioral. Here’s a catalog of some common design patterns within each category, tailored for Rust:

## [Creational Patterns](https://github.com/0xkelvin/Design_Pattern_in_Rust/tree/main/Creational)

1 - [Singleton](https://github.com/0xkelvin/Design_Pattern_in_Rust/tree/main/Creational/example_singleton_pattern) : Ensures a class has only one instance and provides a global point of access to it

2 -  [Factory Method](https://github.com/0xkelvin/Design_Pattern_in_Rust/tree/main/Creational/example_factory_method) : Defines an interface for creating an object, but lets subclasses alter the type of objects that will be created

3 - [Abstract Factory](https://github.com/0xkelvin/Design_Pattern_in_Rust/tree/main/Creational/example_abstract_factory) : Provides an interface for creating families of related or dependent objects without specifying their concrete classes

4 - [Builder](https://github.com/0xkelvin/Design_Pattern_in_Rust/tree/main/Creational/example_builder_pattern) : Separates the construction of a complex object from its representation, allowing the same construction process to create different representations

5 - Prototype : Specifies the kinds of objects to create using a prototypical instance and creates new objects by copying this prototype



## Structural Patterns
1 - Adapter : Converts the interface of a class into another interface clients expect. It lets classes work together that couldn’t otherwise because of incompatible interfaces

2 - Bridge : Decouples an abstraction from its implementation so that the two can vary independently

3 - Composite : Composes objects into tree structures to represent part-whole hierarchies. It lets clients treat individual objects and compositions of objects uniformly

4 - Decorator : Attaches additional responsibilities to an object dynamically. Decorators provide a flexible alternative to subclassing for extending functionality

5 - Facade : Provides a unified interface to a set of interfaces in a subsystem, making it easier to use

6 - Flyweight : Uses sharing to support large numbers of fine-grained objects efficiently

7 - Proxy : Provides a surrogate or placeholder for another object to control access to it

## Behavioral Patterns
1 - Chain of Responsibility : Passes a request along a chain of handlers. Each handler either handles the request or passes it to the next handler in the chain

2 - Command : Encapsulates a request as an object, thereby allowing for parameterization of clients with different requests, queueing of requests, and logging of the requests

3 - Interpreter : Given a language, defines a representation for its grammar along with an interpreter that uses the representation to interpret sentences in the language

4 - Iterator : Provides a way to access the elements of an aggregate object sequentially without exposing its underlying representation

5 - Mediator : Defines an object that encapsulates how a set of objects interact. Promotes loose coupling by keeping objects from referring to each other explicitly

6 - Memento : Without violating encapsulation, captures and externalizes an object’s internal state so that the object can be restored to this state later

7 - Observer : Defines a one-to-many dependency between objects so that when one object changes state, all its dependents are notified and updated automatically

8 - State : Allows an object to alter its behavior when its internal state changes. The object will appear to change its class

9 - Strategy : Defines a family of algorithms, encapsulates each one, and makes them interchangeable. Lets the algorithm vary independently from clients that use it

10 - Template Method : Defines the skeleton of an algorithm in an operation, deferring some steps to subclasses. Lets subclasses redefine certain steps of an algorithm without changing the algorithm’s structure

11 - Visitor : Represents an operation to be performed on the elements of an object structure. Lets you define a new operation without changing the classes of the elements on which it operates.
