# execute the script with the updated map_inventaire.xlsx in the root projet excel_to_sql (and launch the DB see [docker readme](../rust/README.Docker.md) with docker). Make sure to add this .env file as well
```bash
DATABASE_URL=postgres://some-postgres:mysecretpassword@localhost:5432/some-postgres

NAME_MAX_LEN=30
```
## launch the script to add data to DB after clearing all entries (it wont duplicate the data tho)
```bash
cargo run main
```
### you should see all the entries that failed to bee entered and the new ones that are added
