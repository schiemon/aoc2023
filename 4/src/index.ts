import {open} from 'node:fs/promises';
import {FileHandle} from "fs/promises";

type Card = {
    id: number,
    winningNumbers: Set<number>,
    ownNumbers: Set<number>,
}


function parseCard(cardStr: string): Card | undefined {
    const [cardNameAndWinningNumbers, ownNumbers] = cardStr.split("|");

    if (cardNameAndWinningNumbers == null || ownNumbers == null) return undefined;

    const [cardName, winningNumbers] = cardNameAndWinningNumbers.split(":");

    if (cardName == null || winningNumbers == null) return undefined;

    const cardId = Number(cardName.split("Card")[1]);

    if (isNaN(cardId)) return undefined;

    return {
        id: cardId,
        winningNumbers: new Set(winningNumbers.trim().split(" ").filter(numberStr => numberStr !== "").map(Number)),
        ownNumbers: new Set(ownNumbers.trim().split(" ").filter(numberStr => numberStr !== "").map(Number))
    }
}

function getOwnWinningNumbers(card: Card): Set<number> {
    const smallerSet = card.winningNumbers.size < card.ownNumbers.size ? card.winningNumbers : card.ownNumbers;
    const largerSet = smallerSet === card.winningNumbers ? card.ownNumbers : card.winningNumbers;

    const ownWinningNumbers = new Set<number>();

    for (const number of smallerSet) {
        if (largerSet.has(number)) {
            ownWinningNumbers.add(number);
        }
    }

    return ownWinningNumbers;
}

async function part1(file: FileHandle): Promise<number> {
    let totalPoints = 0;

    for await (const line of file.readLines()) {
        const card = parseCard(line);

        if (!card) {
            throw new Error("Invalid card");
        }

        const numOfOwnWinningNumbers = getOwnWinningNumbers(card).size;
        if (numOfOwnWinningNumbers > 0)
            totalPoints += 1 << (numOfOwnWinningNumbers - 1);
    }

    return totalPoints;
}

async function part2(file: FileHandle): Promise<number> {
    let totalScratchCards = 0;

    const numCopiesOfCard: Record<number, number> = {};

    for await (const line of file.readLines()) {
        const card = parseCard(line);

        if (!card) {
            throw new Error("Invalid card");
        }

        numCopiesOfCard[card.id] = numCopiesOfCard[card.id] ?? 0;
        totalScratchCards += 1 + numCopiesOfCard[card.id]!;

        const numOfOwnWinningNumbers = getOwnWinningNumbers(card).size;

        // TODO: get a bound on the number of cards and try to improve the upper bound of this loop.
        for (let ithBelowCard = 1; ithBelowCard <= numOfOwnWinningNumbers; ithBelowCard++) {
            numCopiesOfCard[card.id + ithBelowCard] = numCopiesOfCard[card.id + ithBelowCard] ?? 0;
            numCopiesOfCard[card.id + ithBelowCard] += 1 + numCopiesOfCard[card.id]!;
        }
    }

    return totalScratchCards;
}

async function main() {
    const input_file = process.argv[2];

    if (!input_file) throw new Error("No input file specified");


    console.log("Part 1", await part1(await open(input_file, "r")));
    console.log("Part 2", await part2(await open(input_file, "r")));
}

await main();