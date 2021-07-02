SQLITE="/usr/local/opt/sqlite3/bin/sqlite3"

rm cts.sqlite3
$SQLITE cts.sqlite3 < import_cases.sql
