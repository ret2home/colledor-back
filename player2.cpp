#pragma once
#include "game.hpp"

namespace Player2
{
    void init(Game game)
    {
    }

    Act act(Act previous_act)
    {
        return Act{5, 0, 0};
    }
}