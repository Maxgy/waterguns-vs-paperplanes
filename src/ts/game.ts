import { Game } from "waterguns-vs-paperplanes";

(() => {
    let game = Game.new();

    // render the actual game
    function gameLoop() {
        game.draw();

        window.requestAnimationFrame(gameLoop);
    }
    gameLoop();

    // warn user when reloading
    window.onbeforeunload = function () {
        return "reloading";
    };
})();