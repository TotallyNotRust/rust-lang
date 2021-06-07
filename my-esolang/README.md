# simple-assembly-lang
 
# Commands
> ## PUSH {number 0-127}
> Pushes a number to the stack  

> ## PUSHB {number 0-127 | Option}
> Pushes a number to the bottom of the stack  
> ### Options  
>> @t; adds the top value of the stack to the bottom of the stack

> ## MIN
> Subtracts from the iterator  
> Useful for for loops!  

> ## OUT {Option}
> Outputs the top value of the stack as ascii  
> ### Options
>> @n; number  
>> @c; char  

> ## IN {Option}
> Takes a single char or number input from user and puts it at the top of the stack  
> ### Options
>> @n; number  
>> @c; char  

> ## OLOOP {options}
> Opens loop  
> If no value is passed it will default to @z  
> ### Options
>> @e; will close when stack is empty  
>> @z; will close when the top value of the stack is 0  
>> @iz; will close when iterator is 0  

> ## CLOOP
> Closes loop if the designated loop value is set  

> ## LDITER
> Loads the top value as the stack into the iterator  
> ## CLRITER
> Clears the iterator, setting it to zero  

> ## SET {name} {value}
> Defines a variable named {name} with the value {value}  

> ## GET {name}
> Gets value of variable {name}  
