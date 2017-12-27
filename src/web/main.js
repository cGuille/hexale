import elem from './elem';

const choiceInput = elem.id('hexale-game-word-choice-input');
const choiceConfirmBtn = elem.id('hexale-game-word-choice-confirm-btn');

const guessesDisplay = elem.id('hexale-game-guesses-display');
const guessInput = elem.id('hexale-game-guess-input');
const sendGuessBtn = elem.id('hexale-game-guess-send-btn');

const guessResultTpl = elem.id('hexale-game-guess-result-tpl');

function addGuessResultToDisplay(guessResult) {
    const guessResultElt = elem.instanciate(guessResultTpl, {
        '.hexale-game-guess-value': guessResult.value,
        '.hexale-game-guess-score': guessResult.score,
    });

    if (!guessesDisplay.childElementCount) {
        guessesDisplay.appendChild(guessResultElt);
    } else {
        guessesDisplay.insertBefore(guessResultElt, guessesDisplay.firstElementChild);
    }
}

choiceConfirmBtn.addEventListener('click', event => {
    // TODO:
    // disable choice input
    // display please wait for opponent message
    // send choice to server
    // on error: enable choice input and remove please wait message, show error somehow
    // start polling for opponent ready

    console.log(choiceInput.value);
}, false);

sendGuessBtn.addEventListener('click', event => {
    // TODO:
    // disable guess send
    // display guess in guesses display, empty guess input
    // send guess to server
    // on error: remove guess from display and show error somehow, put back guess in guess input
    // add score to last guess in guess display
    // enable guess send and update guess count

    const dummyGuessResult = { value: guessInput.value, score: 0 };
    addGuessResultToDisplay(dummyGuessResult);

    guessInput.value = null;
}, false);
