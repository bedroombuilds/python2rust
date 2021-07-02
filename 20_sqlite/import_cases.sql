.import --csv case_time_series.csv cases_csv

ALTER TABLE cases_csv DROP COLUMN "Date";

CREATE TABLE cases(
  d timestamp,
  daily_confirmed integer,
  total_confirmed integer,
  daily_recovered integer,
  total_recovered integer,
  daily_deceased integer,
  total_deceased integer
);

insert into cases
select * from cases_csv;

drop table cases_csv;

CREATE TABLE month_cases (
  mon text primary key,
  new_cases integer,
  recovered integer
);
