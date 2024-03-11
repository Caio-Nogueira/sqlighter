# SQLighter

Based on a guide from [build-your-own-x](https://cstack.github.io/db_tutorial/)

The idea is to create a very simple SQLite clone from scratch, hoping to better understand database patterns and how data is persisted to the filesystem. I opted for SQLite, since it is the simpler DBMS → consists of a single file kept in the OS. 

The code is fully implemented in Rust, since I believe it to be the more suitable tool for the job, and I want to become better at it.

The compiler should be simple, as we are not hoping to write complex queries against this db. Throughout the development of this project, the goal should be obtaining a better grasp regarding some important database mechanisms:

- How do transactions get rollbacked?
- When is the data moved from memory to disk?
- Which data structures are used to store data?
- How does Primary Key work?
- What format is the `Prepare` statement saved in?
- How are indexes formatted?

