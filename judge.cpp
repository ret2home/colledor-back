#include "player1.cpp"
#include "player2.cpp"

random_device seed_gen;
mt19937 engine(seed_gen());

ofstream ofs("judge_opt.txt");

Game generate_case()
{
    Game game;
    for (int i = 0; i < TABLE_SIZE; i++)
    {
        for (int j = 0; j < TABLE_SIZE; j++)
        {
            if (TABLE_SIZE / 2 < i)
                game.apple[i][j] = game.apple[TABLE_SIZE - 1 - i][j];
            else if (!(i == 0 && j == TABLE_SIZE / 2) && !(i == TABLE_SIZE - 1 && j == TABLE_SIZE / 2) &&
                     engine() % 5 < 3)
            {
                game.apple[i][j] = 1;
                game.apple_cnt += 2;
                if (i == TABLE_SIZE / 2)
                    game.apple_cnt--;
            }
            ofs << game.apple[i][j];
        }
        ofs << endl;
    }
    return game;
}

string interact(Game game)
{
    double tim[2] = {0, 0};
    chrono::system_clock::time_point clk = chrono::system_clock::now();

    Act previous = {-1, -1, -1};
    for (int t = 0; t < 200; t++)
    {
        cout << "turn: " << t << endl;
        game.printBoard();
        Act act;
        if (!game.turn)
            act = Player1::act(previous);
        else
            act = Player2::act(previous);

        tim[game.turn] +=
            static_cast<double>(chrono::duration_cast<chrono::microseconds>(
                                    chrono::system_clock::now() - clk)
                                    .count() /
                                1000000.0);

        
        if (tim[game.turn] > 100)
        {
            if (!game.turn)
                return "TLE -";
            else
                return "- TLE";
        }
        

        ofs << act.type << " " << act.x << " " << act.y << endl;

        if (game.applyAct(act))
        {
            if (!game.turn)
                return "WA -";
            else
                return "- WA";
        }

        if (!game.apple_cnt)
            break;

        previous = act;

        clk = chrono::system_clock::now();
    }
    return to_string(game.score[0]) + " " + to_string(game.score[1]);
}
int main()
{
    Game game = generate_case();
    Player1::init(game);
    Player2::init(game);
    ofs << interact(game) << endl;
    ofs << "END" << endl;
}