# simpleurlshortener
Server for implementing a super basic client tracking URL shortener service. actix-web + askama + tokio-postgres stack. 

## Features
- Users: Only users registered on the DB are allowed to create URLs
- Client tracking: IP address and time of visit of clients who click on links are saved to the `hits` table.
- Hits counter: Users can see the total number of hits for each of their generated URLs at `/viz`

## Env Variables
| name | default value | required? | description |
|---|---|---|---|---|
| URL_HOST | *empty string* | optional | domain name that this server is serving on |
| PG_USER | postgres | optional | postgresql user |
| PG_DB | *same as PG_USER* | optional | postgresql database name |
| PG_PASSWORD | *no password* | optional | postgresql password |
| PG_HOST | localhost | optional | postgresql host |
| PG_PORT | 5432 | optional | postgresql port |

## Not Done
- New user creation
- Old URL deletion
- Full data export
- Everything else

## Notes
- Be sure to consult https://docs.rs/tokio-postgres/0.5.5/tokio_postgres/types/trait.ToSql.html and https://docs.rs/postgres/0.15.2/postgres/types/index.html to determine the right rust type to use for your column else you'll probably get a nasty runtime panic.