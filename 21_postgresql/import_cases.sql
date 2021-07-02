drop table cases, month_cases;
begin;

CREATE TABLE cases(
  human_date text,
  d date,
  daily_confirmed integer,
  total_confirmed integer,
  daily_recovered integer,
  total_recovered integer,
  daily_deceased integer,
  total_deceased integer
);

\copy cases from 'case_time_series.csv' with csv  header;

ALTER TABLE cases DROP COLUMN "human_date";

CREATE TABLE month_cases (
  mon text primary key,
  new_cases integer,
  recovered integer
);
commit;
