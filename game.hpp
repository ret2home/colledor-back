#pragma once
#include <bits/stdc++.h>
using namespace std;

/*
    type0: 普通の移動
    x: 移動先の x 座標
    y: 移動先の y 座標

    type1: 拠点間の移動
    x: 移動先の x 座標
    y: 移動先の y 座標

    type2: 拠点設置
    x: 設置先の x 座標
    y: 設置先の y 座標

    type3: 水平方向壁設置
    x: 設置先の x 座標
    y: 設置先の y 座標

    type4: 垂直方向壁設置
    x: 設置先の x 座標
    y: 設置先の y 座標

    type5: パス
*/
struct Act
{
    int type, x, y;
};

const int TABLE_SIZE = 9;
const int LEAP_LIMIT = 3;
struct Game
{

    int turn = 0, apple_cnt = 0;

    // 9x9 の配列
    // リンゴがあるかどうか, 水平方向の壁があるかどうか, 垂直方向の壁があるかどうか
    vector<vector<bool>> apple, wall_hrz, wall_vert;

    // 0: 何もない
    // 1: 先手の拠点
    // 2: 後手の拠点
    vector<vector<int>> base;

    // 得点
    // [0]: 先手, [1]: 後手
    vector<int> score;

    // 現在の位置
    // [0]: 先手, [1]: 後手
    vector<pair<int, int>> loc;

    bool isNG(Act act)
    {
        if (act.type < 0 || act.type > 5)
            return 1;

        // 普通の移動
        if (act.type == 0)
        {
            // 場外
            if (act.x < 0 || act.y < 0 || act.x >= TABLE_SIZE || act.y >= TABLE_SIZE)
                return 1;

            // 距離が 1 ではない
            int dis = abs(loc[turn].first - act.x) + abs(loc[turn].second - act.y);
            if (dis != 1)
                return 1;

            // 壁がある
            if (act.x > loc[turn].first && wall_hrz[loc[turn].first][loc[turn].second])
                return 1;
            if (act.x < loc[turn].first && wall_hrz[loc[turn].first - 1][loc[turn].second])
                return 1;
            if (act.y < loc[turn].second && wall_vert[loc[turn].first][loc[turn].second - 1])
                return 1;
            if (act.y > loc[turn].second && wall_vert[loc[turn].first][loc[turn].second])
                return 1;
        }

        // 拠点間の移動
        if (act.type == 1)
        {
            // 場外
            if (act.x < 0 || act.y < 0 || act.x >= TABLE_SIZE || act.y >= TABLE_SIZE)
                return 1;

            // 距離が限界を超えている
            int dis_x = abs(loc[turn].first - act.x), dis_y = abs(loc[turn].second - act.y);
            if (dis_x * dis_x + dis_y * dis_y > LEAP_LIMIT * LEAP_LIMIT || dis_x + dis_y == 0)
                return 1;

            // 拠点間の移動ではない
            if (base[loc[turn].first][loc[turn].second] != turn + 1 || base[act.x][act.y] != turn + 1)
                return 1;
        }
        // 拠点設置
        if (act.type == 2)
        {
            // 場外
            if (act.x < 0 || act.y < 0 || act.x >= TABLE_SIZE || act.y >= TABLE_SIZE)
                return 1;

            // 拠点設置済み
            if (base[act.x][act.y])
                return 1;

            // ポイントが足りない
            if (score[turn] < 3)
                return 1;
        }
        // 水平方向壁設置
        if (act.type == 3)
        {
            // 場外
            if (act.x < 0 || act.y < 0 || act.x >= TABLE_SIZE - 1 || act.y >= TABLE_SIZE - 1)
                return 1;

            // 設置済み
            if (wall_hrz[act.x][act.y] || wall_hrz[act.x][act.y + 1])
                return 1;

            // ポイントが足りない
            if (score[turn] < 3)
                return 1;
        }
        // 垂直方向壁設置
        if (act.type == 4)
        {
            // 場外
            if (act.x < 0 || act.y < 0 || act.x >= TABLE_SIZE - 1 || act.y >= TABLE_SIZE - 1)
                return 1;

            // 設置済み
            if (wall_vert[act.x][act.y] || wall_vert[act.x + 1][act.y])
                return 1;

            // ポイントが足りない
            if (score[turn] < 3)
                return 1;
        }
        return 0;
    }

    bool applyAct(Act act)
    {
        if (isNG(act))
            return 1;

        // 普通の移動
        if (act.type == 0)
        {
            // りんごがある場合
            if (apple[act.x][act.y])
            {
                score[turn] += 5;
                apple[act.x][act.y] = 0;
                apple_cnt--;
            }

            loc[turn].first = act.x;
            loc[turn].second = act.y;
        }

        // 拠点間の移動
        if (act.type == 1)
        {
            // りんごがある場合
            if (apple[act.x][act.y])
            {
                score[turn] += 5;
                apple[act.x][act.y] = 0;
                apple_cnt--;
            }
            
            loc[turn].first = act.x;
            loc[turn].second = act.y;
        }
        // 拠点設置
        if (act.type == 2)
        {
            base[act.x][act.y] = 1 + turn;
            score[turn] -= 3;
        }
        // 水平方向壁設置
        if (act.type == 3)
        {
            wall_hrz[act.x][act.y] = wall_hrz[act.x][act.y + 1] = true;
            score[turn] -= 3;
        }
        // 垂直方向壁設置
        if (act.type == 4)
        {
            wall_vert[act.x][act.y] = wall_vert[act.x + 1][act.y] = true;
            score[turn] -= 3;
        }
        turn ^= 1;
        return 0;
    }

    void printBoard()
    {
        cout << "Point: " << score[0] << ":" << score[1] << endl
             << endl;
        for (int i = 0; i < TABLE_SIZE; i++)
        {
            for (int j = 0; j < TABLE_SIZE; j++)
            {
                if (base[i][j] == 1)
                {
                    cout << "\033[31mF\033[m";
                }
                else if (base[i][j] == 2)
                {
                    cout << "\033[34mF\033[m";
                }
                else
                {
                    cout << " ";
                }

                if (loc[0].first == i && loc[0].second == j)
                    cout << "\033[31mA\033[m";
                else if (loc[1].first == i && loc[1].second == j)
                    cout << "\033[34mB\033[m";
                else if (apple[i][j] == 0)
                    cout << ".";
                else if (apple[i][j] == 1)
                    cout << "#";

                if (j < TABLE_SIZE - 1 && wall_vert[i][j])
                    cout << "|";
                else
                    cout << " ";
            }
            cout << endl;

            if (i == TABLE_SIZE - 1)
                break;

            for (int j = 0; j < 9; j++)
            {
                if (j)
                    cout << " ";
                if (wall_hrz[i][j])
                    cout << "--";
                else
                    cout << "  ";
            }
            cout << endl;
        }
    }

    Game()
    {
        apple.resize(TABLE_SIZE, vector<bool>(TABLE_SIZE));
        wall_hrz.resize(TABLE_SIZE, vector<bool>(TABLE_SIZE));
        wall_vert.resize(TABLE_SIZE, vector<bool>(TABLE_SIZE));
        base.resize(TABLE_SIZE, vector<int>(TABLE_SIZE));
        score.resize(2);
        loc.resize(2);
        loc[0] = {0, TABLE_SIZE / 2};
        loc[1] = {TABLE_SIZE - 1, TABLE_SIZE / 2};
    }
};