#!/bin/bash

cargo install sqlx-cli --no-default-features --features native-tls,postgres

# sqlx db create
# sqlx migrate add create_users_table
# sqlx migrate run
# sqlx migrate revert
# sqlx migrate info
