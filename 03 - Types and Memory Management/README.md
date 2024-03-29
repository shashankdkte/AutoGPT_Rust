# Types and Memory Management

## Language Comparison 
![image info](./pictures/memory_compare.png)

## Integer Types 
![image info](./pictures/integer_types.png)

## DataTypes Cheatsheet
![image info](./pictures/datatype_cheat.png)


## Stack vs Heap
### Stack Call
![image info](./pictures/stack_call.png)

### Stack Storage
![image info](./pictures/stack_storage.png)

### Object Types
![image info](./pictures/object_types.png)

## Stack Information

| Size       	| Fixed (known at compile time)                            	    |
|------------	|--------------------------------------------------------------	|
| Speed      	| Faster the Heap                                          	    |
| Usage      	| Local Variables<br> Function call data                       	|
| Management 	| Rust:automatic (ownership system) <br>C: automatic(compiler) 	|
| Scope      	| Within scope where declared                              	    |

## Heap Information

| Size       	| Dynamic                                                                  	|
|------------	|--------------------------------------------------------------------------	|
| Speed      	| Slower than Stack                                                        	|
| Usage      	| Dynamic Data<br>Large objects that outlive their scope                   	|
| Management 	| Rust:automatic (ownership system)<br>C: manual(new delete, malloc, free) 	|
| Scope      	| Anywhere complying with ownership rules                                  	|


# Ownership and Borrowing

| Ownership                                                       	|
|-----------------------------------------------------------------	|
| Each Value in Rust has a single owner i.e (variable) at a time. 	|
| The value is dropped when owner goes out of scope               	|

| Borrowing                                                  	|
|------------------------------------------------------------	|
| You can have any number of immutable (read-only) refernces 	|
| Refrences must be valid                                    	|