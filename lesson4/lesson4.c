#include <stdio.h>
#include <SDL2/SDL.h>
#include <SDL2_image/SDL_image.h>

const int SCREEN_WIDTH = 640;
const int SCREEN_HEIGHT = 480;

SDL_Texture *load_image(SDL_Renderer *renderer, const char *file) {
    SDL_Texture* tex = 0;
    tex = IMG_LoadTexture(renderer, file);
    if (tex == 0) {
        fprintf(stderr, "Failed to load image: %s\n%s", file ,IMG_GetError());
    }
    return tex;
}

void ApplySurface(int x, int y, SDL_Texture *tex, SDL_Renderer *rend){
    SDL_Rect pos;
    pos.x = x;
    pos.y = y;
    SDL_QueryTexture(tex, NULL, NULL, &pos.w, &pos.h);
    SDL_RenderCopy(rend, tex, NULL, &pos);
}

int main(int argc, char *argv[]) {
    printf("%lu\n", sizeof(SDL_Event));

    if (SDL_Init(SDL_INIT_EVERYTHING) == -1) {
        printf("%s\n", SDL_GetError());
        return 1;
    }
    SDL_Window *window = SDL_CreateWindow("Lesson 4", SDL_WINDOWPOS_CENTERED,
                                                      SDL_WINDOWPOS_CENTERED,
                                                      SCREEN_WIDTH,
                                                      SCREEN_HEIGHT,
                                                      SDL_WINDOW_SHOWN);
    if (!window) {
        printf("%s\n", SDL_GetError());
        return 2;
    }
    SDL_Renderer *renderer = SDL_CreateRenderer(window, -1, SDL_RENDERER_ACCELERATED
                                                          | SDL_RENDERER_PRESENTVSYNC);
    if (!renderer) {
        printf("%s\n", SDL_GetError());
        return 3;
    }
    SDL_Texture *image = load_image(renderer, "res/image.png");

    int quit = 0;
    SDL_Event e;
    int x = 0, y = 0;
    while (!quit) {
        while (SDL_PollEvent(&e)){
            if (e.type == SDL_QUIT) quit = true;
            if (e.type == SDL_KEYDOWN) quit = true;
            if (e.type == SDL_MOUSEBUTTONDOWN) quit = true;
            printf("%d\n", e.type);
        }
        SDL_RenderClear(renderer);
        ApplySurface(x, y, image, renderer);
        SDL_RenderPresent(renderer);
    }

    return 0;
}
