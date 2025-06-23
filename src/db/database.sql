PRAGMA foreign_keys = ON;

CREATE TABLE users (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL,
  email TEXT UNIQUE NOT NULL,
  password TEXT NOT NULL
);

CREATE TABLE groups (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL,
  description TEXT NOT NULL,
  owner_id INTEGER NOT NULL,
  group_start_date TEXT NOT NULL,
  group_end_date TEXT NOT NULL,
  location TEXT NOT NULL,
  FOREIGN KEY (owner_id) REFERENCES users(id)
);

CREATE TABLE group_members (
  group_id INTEGER NOT NULL,
  user_id INTEGER NOT NULL,
  PRIMARY KEY (group_id, user_id),
  FOREIGN KEY (group_id) REFERENCES groups(id),
  FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE TABLE expenses (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  description TEXT,
  amount REAL NOT NULL,
  payer_id INTEGER NOT NULL,
  group_id INTEGER NOT NULL,
  date TEXT NOT NULL,
  FOREIGN KEY (payer_id) REFERENCES users(id),
  FOREIGN KEY (group_id, payer_id) REFERENCES group_members(group_id, user_id),
  FOREIGN KEY (group_id) REFERENCES groups(id)
);

CREATE TABLE expense_participants (
  expense_id INTEGER NOT NULL,
  user_id INTEGER NOT NULL,
  PRIMARY KEY (expense_id, user_id),
  FOREIGN KEY (expense_id) REFERENCES expenses(id),
  FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE TABLE transactions (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  payer_id INTEGER NOT NULL,
  receiver_id INTEGER NOT NULL,
  amount REAL NOT NULL,
  date TEXT NOT NULL,
  status TEXT NOT NULL CHECK (status IN ('pending', 'completed')),
  group_id INTEGER NOT NULL,
  FOREIGN KEY (payer_id) REFERENCES users(id),
  FOREIGN KEY (receiver_id) REFERENCES users(id),
  FOREIGN KEY (group_id) REFERENCES groups(id),
  FOREIGN KEY (group_id, payer_id) REFERENCES group_members(group_id, user_id),
  FOREIGN KEY (group_id, receiver_id) REFERENCES group_members(group_id, user_id)
);

CREATE TABLE api_tokens (
  token TEXT PRIMARY KEY,
  user_id INTEGER NOT NULL,
  FOREIGN KEY (user_id) REFERENCES users(id)
);