// Compile with:
//    emcc fullscreen.c -s USE_SDL=2 -s ASYNCIFY -s ALLOW_MEMORY_GROWTH=1
// Run with:
//    emrun index_fullscreen.html

#include <stdbool.h>
#include <stdlib.h>
#include <stdio.h>
#include <SDL2/SDL.h>

int main(int argc, char ** argv)
{
    bool quit = false;
    SDL_Event event;

    SDL_Init(SDL_INIT_VIDEO);
    SDL_Window *window = SDL_CreateWindow("Testing threads, SDL2, WASM",
        SDL_WINDOWPOS_UNDEFINED, SDL_WINDOWPOS_UNDEFINED, 640, 480, 0);
    SDL_Renderer * renderer = SDL_CreateRenderer(window, -1, 0);

    int mouse_x = 0, mouse_y = 0;

    while (!quit)
    {
        SDL_WaitEvent(&event);

        switch (event.type)
        {
            case SDL_QUIT:
                quit = true;
                break;
            case SDL_MOUSEMOTION:
                mouse_x = event.motion.x;
                mouse_y = event.motion.y;
                break;
        }

        SDL_SetRenderDrawColor(renderer, 230, 230, 230, 255);
        SDL_RenderClear(renderer);
        SDL_SetRenderDrawColor(renderer, 255, 0, 0, 255);
        SDL_RenderDrawLine(renderer, 0, 0, mouse_x, mouse_y);
        SDL_RenderPresent(renderer);
    }

    SDL_DestroyRenderer(renderer);
    SDL_DestroyWindow(window);
    SDL_Quit();

    return EXIT_SUCCESS;
}
