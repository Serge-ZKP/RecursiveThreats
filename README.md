In this example, we define a function recursive_factorial that computes the factorial of a given integer n using recursion. However, instead of making the recursive calls directly, we create a new thread for each recursive call.

For each recursive call, we create a new channel with std::sync::mpsc::channel() to communicate the result back to the parent thread. We then spawn a new child thread with thread::spawn() and pass in a closure that computes the child's result and sends it back through the channel.

After starting the child thread, the parent thread waits for the result to be sent back through the channel using rx.recv().unwrap(). Finally, we join the child thread to wait for it to complete and return the child's result to the parent.
