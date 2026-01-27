# Advanced OOP Questions & Answers

## 1. Composition vs. Inheritance: When to use which?
**Answer**: 
- **Inheritance** represents an "is-a" relationship (e.g., a Dog is an Animal). It is useful for code reuse but can lead to tight coupling and deep hierarchies.
- **Composition** represents a "has-a" relationship (e.g., a Car has an Engine). It is generally preferred because it provides more flexibility, loose coupling, and allows changing behavior at runtime.

## 2. What are SOLID principles?
**Answer**:
- **S**ingle Responsibility: A class should have only one reason to change.
- **O**pen/Closed: Software entities should be open for extension but closed for modification.
- **L**iskov Substitution: Objects of a superclass should be replaceable with objects of its subclasses without affecting correctness.
- **I**nterface Segregation: Clients should not be forced to depend on interfaces they do not use.
- **D**ependency Inversion: Depend on abstractions, not concretions.

## 3. Difference between Abstract Class and Interface?
**Answer**:
- **Abstract Class**: Can have both abstract (without body) and concrete methods. Can have state (fields). A class can inherit only one abstract class. Represents "identity".
- **Interface**: Traditionally only had abstract methods (though modern languages allow default methods). Cannot have state. A class can implement multiple interfaces. Represents "capability".

## 4. What is the "Diamond Problem" and how is it solved?
**Answer**: It occurs in multiple inheritance where a class inherits from two classes that both inherit from a common base class. This leads to ambiguity. Many languages (like Java) solve this by disallowing multiple inheritance of classes and using Interfaces instead. Languages like Python use Method Resolution Order (MRO).
