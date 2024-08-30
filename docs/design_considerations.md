# Goal

The goal of this rewrite is to split the convoluted responsibilities of the `isotovideo` module
into separate, specialized modules. Each of these libraries should only be responsible for its
own set usecase and posess as small of an interface as posible.

That means that `isotomachine`, `isotoenv` and `isototest` must not rely on each other to
fulfill their designated area of responsibility.
Neither should they interact with each other or the output of any of the other libraries.
If functionality must be shared between these libraries (like VNC connections) it should be split off
into a separate supporting library to prevent repitition and "cross contamination".
