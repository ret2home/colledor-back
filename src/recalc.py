import psycopg2
import os

DB_URL = os.environ["DATABASE_URL"]
CONTEST_START = int(os.environ["CONTEST_START"])

con = psycopg2.connect(DB_URL)
cur = con.cursor()
cur.execute("BEGIN;")

cur.execute("DELETE FROM ratinghistory")

ratings = {}
cur.execute("SELECT id from users", ())
for user in cur.fetchall():
    ratings[user[0]] = 1500
    cur.execute("INSERT INTO ratinghistory VALUES(%s,%s,1500)",
                (CONTEST_START, user[0]))

cur.execute("SELECT end_num,user1_id,user2_id,user1_score,user2_score FROM challenges WHERE stat='FINISHED' AND rated=1 ORDER BY end_num")
for challenge in cur.fetchall():
    end_num = challenge[0]
    user1 = challenge[1]
    user1_rate = ratings[user1]
    user2 = challenge[2]
    user2_rate = ratings[user2]
    user1_score = challenge[3]
    user2_score = challenge[4]
    win1, win2 = 0, 0
    if user1_score == "TLE" or user1_score == "WA":
        win2 = 1
    elif user2_score == "TLE" or user2_score == "WA":
        win1 = 1
    elif int(user1_score) > int(user2_score):
        win1 = 1
    elif int(user1_score) < int(user2_score):
        win2 = 1
    else:
        win1 = win2 = 0.5
    W = 1/(10**((user2_rate-user1_rate)/400)+1)
    new_user1_rate = round(user1_rate+32*(win1-W))
    new_user2_rate = round(user2_rate+32*(win2-(1-W)))

    ratings[user1] = new_user1_rate
    ratings[user2] = new_user2_rate

    
    cur.execute("INSERT INTO ratinghistory VALUES(%s,%s,%s)",
                (end_num, user1, new_user1_rate))
    cur.execute("INSERT INTO ratinghistory VALUES(%s,%s,%s)",
                (end_num, user2, new_user2_rate))

    cur.execute("UPDATE users SET rating=%s WHERE id=%s",
                (new_user1_rate, user1))
    cur.execute("UPDATE users SET rating=%s WHERE id=%s",
                (new_user2_rate, user2))
    

cur.execute("COMMIT;")
print(ratings)