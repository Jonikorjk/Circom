pragma circom  2.0.0;

include "node_modules/circomlib/circuits/comparators.circom";

template MatrixTranspose() {
    signal input in[9][9];
    signal output out[9][9];

    for (var i = 0; i < 9; i++) {
        for (var j = 0; j < 9; j++) {
            out[j][i] <== in[i][j];
        }
    }
}

template ValidateInputSudokuMatrix() { 
    signal input matrix[9][9];
    component gt[9][9];
    component lt[9][9];

    for (var i = 0; i < 9; i++) { 
        for (var j = 0; j < 9; j++) { 
            gt[i][j] = GreaterThan(0);
            gt[i][j].in <== [matrix[i][j], 0];
            gt[i][j].out === 1;

            lt[i][j] = LessThan(10);
            lt[i][j].in <== [matrix[i][j], 10];
            lt[i][j].out === 1;
        }
    }
}

template VerifySudokuRows() { 
    signal input row[9];
    component isEq[9][9];

    for (var i = 0; i < 9; i++) { 
        for (var j = i; j < 9; j++) { 
            isEq[i][j] = IsEqual();
            isEq[i][j].in <== [row[i], row[j]];
            isEq[i][j].out === 0;
        }
    }
}

template VerifySubGrids() { 
    signal input matrix[9][9];
    component verifyRows[9];

    var gridsIndexes[9][9][2] = [
        [
            [0, 0], [0, 1], [0, 2],
            [1, 0], [1, 1], [1, 2],
            [2, 0], [2, 1], [2, 2]
        ],
        [
            [3, 0], [3, 1], [3, 2],
            [4, 0], [4, 1], [4, 2],
            [5, 0], [5, 1], [5, 2]
        ],
        [
            [6, 0], [6, 1], [6, 2],
            [7, 0], [7, 1], [7, 2],
            [8, 0], [8, 1], [8, 2]
        ],
        //
        [
            [0, 3], [0, 4], [0, 5],
            [1, 3], [1, 4], [1, 5],
            [2, 3], [2, 4], [2, 5]
        ],
        [
            [3, 3], [3, 4], [3, 5],
            [4, 3], [4, 4], [4, 5],
            [5, 3], [5, 4], [5, 5]
        ],
        [
            [6, 3], [6, 4], [6, 5],
            [7, 3], [7, 4], [7, 5],
            [8, 3], [8, 4], [8, 5]
        ],
        //
        [
            [0, 6], [0, 7], [0, 8],
            [1, 6], [1, 7], [1, 8],
            [2, 6], [2, 7], [2, 8]
        ],
        [
            [3, 6], [3, 7], [3, 8],
            [4, 6], [4, 7], [4, 8],
            [5, 6], [5, 7], [5, 8]
        ],
        [
            [6, 6], [6, 7], [6, 8],
            [7, 6], [7, 7], [7, 8],
            [8, 6], [8, 7], [8, 8]
        ]
    ];

    for (var i = 0; i < 9; i++) { 
        verifyRows[i] = VerifySudokuRows();
        for (var j = 0; j < 9; j++) {
            verifyRows[i].row[j] <== matrix[gridsIndexes[i][j][0]][gridsIndexes[i][j][1]];
        }
    }
}

template VerifySudokuMatrix() { 
    signal input matrix[9][9];
    signal transposedMatrix[9][9];

    component matrixTranspose = MatrixTranspose();
    component rowsVerifier[9];
    component colsVerifier[9];
    component subGridVerifier = VerifySubGrids();

    // Verify cols
    for (var i; i < 9; i++) { 
        rowsVerifier[i] = VerifySudokuRows();
        rowsVerifier[i].row <== matrix[i];
    }

    // Transpose sudoku matrix
    matrixTranspose.in <== matrix;
    transposedMatrix <== matrixTranspose.out;

    // Verify Cols
    for (var i; i < 9; i++) { 
        colsVerifier[i] = VerifySudokuRows();
        colsVerifier[i].row <== matrix[i];
    }

    // Verify subgrids
    subGridVerifier.matrix <== matrix;
}

template Main() {
    signal input sudokuMatrix[9][9];

    component sudokuValidator = ValidateInputSudokuMatrix(); 
    sudokuValidator.matrix <== sudokuMatrix;

    component sudokuVerifier = VerifySudokuMatrix();
    sudokuVerifier.matrix <== sudokuMatrix;
}

component main = Main();