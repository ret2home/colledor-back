import os
import psycopg2
import hashlib

DB_URL = os.environ["DATABASE_URL"]

con = psycopg2.connect(DB_URL)
cur = con.cursor()

cur.execute("BEGIN;")

cur.execute('''
CREATE TABLE users(
    id VARCHAR UNIQUE NOT NULL PRIMARY KEY,
    pw VARCHAR NOT NULL,
    rating INT8,
    stock INT8
);
''')

cur.execute('''
CREATE TABLE votes(
    id INT8,
    user_id VARCHAR NOT NULL,
    vote INT8
);
''')


cur.execute('''
CREATE TABLE submissions(
    id INT8 UNIQUE NOT NULL PRIMARY KEY,
    tim_st VARCHAR NOT NULL,
    tim_num INT8 NOT NULL,
    user_id VARCHAR NOT NULL,
    source VARCHAR
);
''')


cur.execute('''
CREATE TABLE challenges(
    id INT8 UNIQUE NOT NULL PRIMARY KEY,
    rated INT8 NOT NULL,
    tim_st VARCHAR NOT NULL,
    tim_num INT8 NOT NULL,
    server_id INT8 NOT NULL,
    stat VARCHAR NOT NULL,
    user1_id VARCHAR NOT NULL,
    user2_id VARCHAR NOT NULL,
    user1_score VARCHAR,
    user2_score VARCHAR,
    user1_vote INT8,
    user2_vote INT8,
    opt VARCHAR,
    end_num INT8 NOT NULL
);
''')

cur.execute('''
CREATE TABLE ratinghistory(
    tim_num INT8 NOT NULL,
    user_id VARCHAR NOT NULL,
    rating INT8 NOT NULL
);
''')

users=open("./users.txt").readlines()
contest_start=int(os.environ["CONTEST_START"])

for user in users:
    x=user.split()
    id=x[0]
    pw=x[1]
    hashed_pw=hashlib.md5(pw.encode()).hexdigest()
    cur.execute("INSERT INTO users VALUES(%s,%s,1500,1500)",(id,hashed_pw,))
    cur.execute("INSERT INTO ratinghistory VALUES(%s,%s,1500)",(contest_start,id,))


cur.execute("COMMIT;")