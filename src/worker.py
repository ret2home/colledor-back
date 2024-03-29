from http import server
import threading
import time
import requests
import psycopg2
import os
import time
import math

DB_URL=os.environ["DATABASE_URL"]
SERVER_URLS=["http://localhost:8080","http://localhost:8080","http://localhost:8080"]

def worker(server_id):
    while True:
        con=psycopg2.connect(DB_URL)
        cur=con.cursor()
        cur.execute("BEGIN;")
        cur.execute("SELECT id,user1_id,user2_id,rated,user1_vote,user2_vote FROM challenges WHERE stat='WJ' AND server_id=%s AND (rated=0 OR tim_num<%s) ORDER BY id LIMIT 1",(server_id,int(time.time())-60))
        unjudged=cur.fetchall()

        if len(unjudged)==0:
            cur.execute("COMMIT;")
            time.sleep(1)
            continue
        
        id=unjudged[0][0]
        user1=unjudged[0][1]
        user2=unjudged[0][2]
        rated=unjudged[0][3]
        user1_vote=unjudged[0][4]
        user2_vote=unjudged[0][5]
        user1_source=""
        user2_source=""
        cur.execute("UPDATE challenges SET stat='RUNNING' WHERE id=%s",(id,))
        cur.execute("COMMIT;")

        cur.execute("SELECT source FROM submissions WHERE user_id=%s ORDER BY id DESC LIMIT 1",(user1,))
        user1_source=cur.fetchone()[0]
        
        cur.execute("SELECT source FROM submissions WHERE user_id=%s ORDER BY id DESC LIMIT 1",(user2,))
        user2_source=cur.fetchone()[0]

        judge_request_data={
            "id": id,
            "user1_source": user1_source,
            "user2_source": user2_source
        }

        while True:
            print("Go")
            start_res=requests.post(f"{SERVER_URLS[server_id]}/judge-request",json=judge_request_data).status_code
            if start_res==200:
                break
            print(start_res)
            time.sleep(1)

        start_time=time.time()

        print(f"START: {id} {time.time()}")

        judge_res=""
        user1_score=""
        user2_score=""
        spl=[]
        while time.time()-start_time<240:
            get_res=requests.get(f"{SERVER_URLS[server_id]}/judge-info/{id}")
            if get_res.status_code!=200:
                time.sleep(1)
                continue
            judge_res=get_res.json()["output"]
            cur.execute("UPDATE challenges SET opt=%s WHERE id=%s",(judge_res,id,))
            spl=judge_res.splitlines()
            if len(spl)!=0 and spl[-1]=="END":
                result=spl[-2].split()
                user1_score=result[0]
                user2_score=result[1]
                break
            time.sleep(1)
        
        if user1_score=="":
            if len(spl)%2:
                user1_score="TLE"
                user2_score="-"
            else:
                user1_score="-"
                user2_score="TLE"

        print(f"FINISH: {id} {time.time()}")


        tim=int(time.time())
        cur.execute("UPDATE challenges SET user1_score=%s , user2_score=%s , stat='FINISHED', end_num=%s WHERE id=%s",(user1_score,user2_score,tim,id,))

        if rated==1:
            cur.execute("BEGIN;")
            cur.execute("SELECT rating FROM users WHERE id=%s",(user1,))
            user1_rate=cur.fetchall()[0][0]
            cur.execute("SELECT rating FROM users WHERE id=%s",(user2,))
            user2_rate=cur.fetchall()[0][0]
            win1,win2=0,0
            if user1_score=="TLE" or user1_score=="WA":
                win2=1
            elif user2_score=="TLE" or user2_score=="WA":
                win1=1
            elif int(user1_score)>int(user2_score):
                win1=1
            elif int(user1_score)<int(user2_score):
                win2=1
            else:
                win1=win2=0.5
            
            W=1/(10**((user2_rate-user1_rate)/400)+1)
            new_user1_rate=round(user1_rate+32*(win1-W))
            new_user2_rate=round(user2_rate+32*(win2-(1-W)))
            cur.execute("INSERT INTO ratinghistory VALUES(%s,%s,%s)",(tim,user1,new_user1_rate))
            cur.execute("INSERT INTO ratinghistory VALUES(%s,%s,%s)",(tim,user2,new_user2_rate))
            cur.execute("UPDATE users SET rating=%s WHERE id=%s",(new_user1_rate,user1))
            cur.execute("UPDATE users SET rating=%s WHERE id=%s",(new_user2_rate,user2))

            if win1==0 and user2_vote!=0:
                cur.execute("UPDATE users SET stock=stock-50 WHERE id in (SELECT user_id FROM votes WHERE id=%s AND vote=0)",(id,))
                prize=math.ceil(50*user1_vote/user2_vote)
                cur.execute("UPDATE users SET stock=stock+%s WHERE id in (SELECT user_id FROM votes WHERE id=%s AND vote=1)",(prize,id))
            
            if win2==0 and user1_vote!=0:
                cur.execute("UPDATE users SET stock=stock-50 WHERE id in (SELECT user_id FROM votes WHERE id=%s AND vote=1)",(id,))
                prize=math.ceil(50*user2_vote/user1_vote)
                cur.execute("UPDATE users SET stock=stock+%s WHERE id in (SELECT user_id FROM votes WHERE id=%s AND vote=0)",(prize,id))
            
            
            cur.execute("COMMIT;")

        while True:
            stat=requests.get(f"{SERVER_URLS[server_id]}/judge-kill/{id}").status_code
            if stat==200:
                break
            time.sleep(1)

threads=[]
for i in range(len(SERVER_URLS)):
    for j in range(1):
        t=threading.Thread(target=worker,args=(i,))
        t.start()
        threads.append(t)

for t in threads:
    t.join()