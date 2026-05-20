# Explanation to myself in the future how everything is working since i am still learning

## May 19

User run the program using:

```
rust cargo run -- argument1 argument2 
```

where:
- argument1 is path
- argument2 is query

then i use std::env to collect the args

it stored in a vector of String type

text 0th index -> program name 1st index -> path 2nd index -> query 

then i pass it to the config module

it takes &[String]

here happen the 'Deref Coercion'

i pass the reference from main file and in the build function it takes the slice of the vector string

basically what i understand by now is that there is a whole vector reference passed and it takes just a string and don't own the data

basically it borrow the string reference so the function can perform action based on it

---

### Inside the config file

i created a struct which have a lifetime and consist of two fields path and query which stores the reference &str

then a impl block for the config which contains the function build

in which i takes the &[String] args

and it return a Result

now result return two things

first is actually what we want and second is err

so in rust term Ok() or Err()

and as of Result it's syntax is:

rust Result<T, E> 

if len is less than 3 then err

then i am doing:

rust &args[index] 

which contains a &String

a reference to the string stored in the vector args

but i am storing it in a variable:

rust path: &str 

where the concept of Deref Coercion again used

the Type was &String but rust internally converts it to &str and store it in the variable

in the last line i return an Ok which contains the Config

and since we are using the impl of Config i don't need to type Config again and again inside that block

i can just use Self and it will refer to the Config struct because the impl block is made for that

---

# In Professional Terms

- Vec<String> owns all command line arguments.
- Config does not own the arguments. It borrows string slices (&str) from the vector.
- The lifetime 'args ensures that Config cannot outlive the original arguments vector.
- &[String] is a slice (borrowed view) into the vector instead of taking ownership of the whole vector.
- Result<T, E> is Rust’s standard recoverable error handling type:
  - Ok(T) → success value
  - Err(E) → failure value
- panic!() crashes the program and is generally avoided for normal user input mistakes.
- Deref coercion automatically converts:
  
rust &String -> &str 

because String implements the Deref trait targeting str.

- Self inside an impl block refers to the type currently being implemented.
- The current Config struct acts as a lightweight borrowed view into command line arguments rather than creating new owned strings.


## May 20 


# In Professional Terms

- search() borrows the original file contents instead of creating new owned strings.
- .lines() creates an iterator over borrowed string slices (&str) from the original data.
- .enumerate() transforms iterator items from:
  
rust &str 

to:

rust (usize, &str) 

where:
  - usize represents the line index
  - &str represents the line content

- .filter() retains only iterator items matching the query condition.
- .collect() consumes the iterator and builds a concrete collection type.
- The iterator item type determines what collect() can construct.
- Because enumerate() produces tuples, the final collection type must be:

rust Vec<(usize, &str)> 

instead of:

rust Vec<&str> 

- Tuple destructuring allows direct extraction of tuple elements:

rust for (index, value) in ans 

- Rust compiler diagnostics often describe iterator type transformations step-by-step through the method chain.
- enumerate() starts indexing from 0, matching Rust’s standard indexing model.
- The lifetime 'matches ensures returned string slices cannot outlive the original file content buffer.
- The search implementation avoids unnecessary allocations because all matching lines are borrowed references into the original file contents.