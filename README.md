# honote
write it later


# installation 
We assume your computer is installed rust and sql server.

Now, we are using SQLite for developing. If you chose some other database, you need fix some code and sql syntax.

```
$ diesel setup --database-url=./db/honey.db 
$ diesel migration run --database-url=./db/honey.db  
```