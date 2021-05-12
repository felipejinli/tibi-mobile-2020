CREATE TABLE users (
  id VARCHAR PRIMARY KEY, -- This is the student's UPI
  given_name VARCHAR NOT NULL,
  full_name VARCHAR NOT NULL,
  username VARCHAR NOT NULL,
  email VARCHAR NOT NULL,
  department VARCHAR NOT NULL,
  is_student BOOLEAN NOT NULL,
  is_admin BOOLEAN NOT NULL DEFAULT false
)
