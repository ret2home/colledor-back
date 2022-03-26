import os
import psycopg2
import hashlib

DB_URL = os.environ["DATABASE_URL"]

con = psycopg2.connect(DB_URL)
cur = con.cursor()

cur.execute("BEGIN;")

try:
    cur.execute("DROP TABLE users;")
except:
    pass

cur.execute('''
CREATE TABLE users(
    id VARCHAR UNIQUE NOT NULL PRIMARY KEY,
    pw VARCHAR NOT NULL,
    rating INTEGER
);
''')

try:
    cur.execute("DROP TABLE submissions;")
except:
    pass

cur.execute('''
CREATE TABLE submissions(
    id INTEGER UNIQUE NOT NULL PRIMARY KEY,
    tim_st VARCHAR NOT NULL,
    tim_num INTEGER NOT NULL,
    user_id VARCHAR NOT NULL,
    source VARCHAR
);
''')

try:
    cur.execute("DROP TABLE challenges;")
except:
    pass

cur.execute('''
CREATE TABLE challenges(
    id INTEGER UNIQUE NOT NULL PRIMARY KEY,
    rated INTEGER NOT NULL,
    tim_st VARCHAR NOT NULL,
    tim_num INTEGER NOT NULL,
    server_id INTEGER NOT NULL,
    stat VARCHAR NOT NULL,
    user1_id VARCHAR NOT NULL,
    user2_id VARCHAR NOT NULL,
    user1_score VARCHAR,
    user2_score VARCHAR,
    opt VARCHAR,
    end_num INTEGER NOT NULL
);
''')

try:
    cur.execute("DROP TABLE ratinghistory;")
except:
    pass

cur.execute('''
CREATE TABLE ratinghistory(
    tim_num INTEGER NOT NULL,
    user_id VARCHAR NOT NULL,
    rating INTEGER NOT NULL
);
''')

users=open("./users.txt").readlines()
contest_start=int(os.environ["CONTEST_START"])

for user in users:
    x=user.split()
    id=x[0]
    pw=x[1]
    hashed_pw=hashlib.md5(pw.encode()).hexdigest()
    cur.execute("INSERT INTO users VALUES(%s,%s,1500)",(id,hashed_pw,))
    cur.execute("INSERT INTO ratinghistory VALUES(%s,%s,1500)",(contest_start,id,))

cur.execute("COMMIT;")