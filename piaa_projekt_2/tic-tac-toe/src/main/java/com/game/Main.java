package com.game;

import javafx.application.Application;
import javafx.scene.*;
import javafx.scene.paint.*;
import javafx.scene.shape.*;
import javafx.stage.*;
import javafx.scene.layout.*;
import javafx.geometry.*;
import javafx.scene.control.*;
import javafx.collections.*;

enum GameMode{
    GAME_START,
    PVP,
    PVE
}

public class Main extends Application {
    public static void main(String[] args) throws Exception {
            System.err.println("MAIN START");
        launch(args);
    }


    public void start(Stage stage) throws Exception {

        
        Ui ui = new Ui();

        stage.setTitle("Gra");
        stage.setScene(ui.GetScene());
        stage.show();
    }
 
    public void stop() throws Exception {
        System.err.println("stopping");
    }
}

class Ui {
    Scene scene = null;
    Node gameStartView = null;
    Node gamaplayInterfaceView = null;
    int winCondition = 0;
    int boardSize = 0;
    GameMode state = GameMode.GAME_START;
    private int[][] boardState;
    private Button[][] buttons;
    private int currentPlayer = 1; //X - 1, O - 0

    public Ui() {
    this.scene = new Scene(new BorderPane(), 600, 600);
    this.changeScene();
    }

    public Scene GetScene() {
        return this.scene;
    }

public void changeScene() {
    BorderPane root = (BorderPane) scene.getRoot();
    

if (state == GameMode.GAME_START) {
    System.err.println("State = " + state);
    
        root.setCenter(this.getGameStartView());    
    } else {
        System.err.println("State = " + state);
        this.gamaplayInterfaceView = null;
        Node gameplay = getGameplayInterfaceView();
        root.setCenter(gameplay);     
    }
}

    public Node getGameStartView() {
        if (this.gameStartView != null) {
            return this.gameStartView;
        }

        Label labelBoard = new Label("Set board size");
        Slider boardSizeSlider = new Slider(3, 10, 3);

        boardSizeSlider.setShowTickMarks(true);
        boardSizeSlider.setShowTickLabels(true);
        boardSizeSlider.setMajorTickUnit(1); 
        boardSizeSlider.setMinorTickCount(0);
        boardSizeSlider.setSnapToTicks(true);

        Label labelWinCondition = new Label("Set winning streak");
        Slider winSizeSlider = new Slider(3, 10, 3);

        winSizeSlider.setShowTickMarks(true);
        winSizeSlider.setShowTickLabels(true);
        winSizeSlider.setMajorTickUnit(1); 
        winSizeSlider.setMinorTickCount(0);
        winSizeSlider.setSnapToTicks(true);
    
        Label selectModeLabel = new Label();
        selectModeLabel.setTextFill(Color.BLUE);

        Label label = new Label("Click the box to select game mode: ");
        CheckBox checkBx1 = new CheckBox("PVE");
        checkBx1.setSelected(false);

        checkBx1.setOnAction(e -> {
            selectModeLabel.setText("You selected: " + (checkBx1.isSelected() ? "PVE" : "PVP"));
        });



        Button startButton = new Button("Start game!");
        startButton.setOnAction( e -> {
                System.err.println("TEST");
            if(boardSizeSlider.getValue() >= winSizeSlider.getValue()){

                if(checkBx1.isSelected() == false){
                    System.err.println("PVP");
                    this.state = GameMode.PVP;
                } else {
                    System.err.println("PVE");
                    this.state = GameMode.PVE;
                }
                
  
            this.boardSize = (int)boardSizeSlider.getValue();
            this.winCondition = (int)winSizeSlider.getValue();

            System.err.println(this.boardSize);
            System.err.println(this.winCondition);

            this.changeScene();
             
            }else{
                System.err.println("Cannot start game with impossible win condition!");
            }
        
        });
        



        VBox box = new VBox();
        box.setAlignment(Pos.CENTER);
        box.setPadding(new Insets(10));
        box.setSpacing(10);
        box.getChildren().addAll(labelBoard, boardSizeSlider, labelWinCondition, winSizeSlider, label, checkBx1, selectModeLabel, startButton);

        this.gameStartView = box;
        return this.gameStartView;
    }

   public Node getGameplayInterfaceView(){
    System.err.println("DEBUG: Building game with size: " + this.boardSize);
    this.boardState = new int[this.boardSize][this.boardSize];
    this.buttons = new Button[this.boardSize][this.boardSize];
     if (this.gamaplayInterfaceView != null) {
            return this.gamaplayInterfaceView;
        }


    GridPane grid = new GridPane();
    grid.setAlignment(Pos.CENTER);
    grid.setHgap(5);
    grid.setVgap(5);

    for (int row = 0; row < this.boardSize; row++) {
        for (int col = 0; col < this.boardSize; col++) {
            Button btn = new Button("");
            btn.setPrefSize(60, 60); 
            

            final int r = row;
            final int c = col;
            
            btn.setOnAction(e -> handleCellClick(r, c, btn));

            this.buttons[row][col] = btn;
            grid.add(btn, col, row); 
        }
    }
    this.gamaplayInterfaceView = grid;
    return this.gamaplayInterfaceView;
    }

   private void handleCellClick(int r, int c, Button btn) {
    if (boardState[r][c] != 0) return; 

    boardState[r][c] = currentPlayer;
    btn.setText(currentPlayer == 1 ? "X" : "O");

    if (checkWin(currentPlayer)) {
        endGame((currentPlayer == 1 ? "Player X" : "Player O") + " won!");
        return;
    }
    if (isBoardFull()) {
        endGame("Draw!");
        return;
    }

    if (state == GameMode.PVP) {
        currentPlayer = (currentPlayer == 1) ? 2 : 1; 
    } else if (state == GameMode.PVE) {
        botMove(); 
    }
    }
private boolean checkWin(int player) {
    for (int r = 0; r < boardSize; r++) {
        for (int c = 0; c < boardSize; c++) {
            if (boardState[r][c] == player) {
                int[][] dirs = {{0,1}, {1,0}, {1,1}, {1,-1}};
                for (int[] d : dirs) {
                    int count = 1;
                    for (int i = 1; i < winCondition; i++) {
                        int nr = r + d[0] * i, nc = c + d[1] * i;
                        if (nr >= 0 && nr < boardSize && nc >= 0 && nc < boardSize && boardState[nr][nc] == player) count++;
                        else break;
                    }
                    if (count >= winCondition) return true;
                }
            }
        }
    }
    return false;
}
private void botMove() {
    int[] bestMove = findBestMove();
    if (bestMove[0] != -1) {
        boardState[bestMove[0]][bestMove[1]] = 2; // Bot zawsze jako 2
        buttons[bestMove[0]][bestMove[1]].setText("O");
        
        if (checkWin(2)) endGame("Bot have won!");
        else if (isBoardFull()) endGame("Draw!");
    }
}
private boolean isBoardFull() {
    for (int i = 0; i < boardSize; i++) {
        for (int j = 0; j < boardSize; j++) {
            if (boardState[i][j] == 0) return false;
        }
    }
    return true;
}

private void endGame(String msg) {
    this.currentPlayer = 1; 
    Alert alert = new Alert(Alert.AlertType.INFORMATION);
    alert.setContentText(msg);
    alert.showAndWait();


    this.state = GameMode.GAME_START;
    this.gameStartView = null; 
    this.gamaplayInterfaceView = null;
    this.changeScene();
}
private int[] findBestMove() {
    for (int r = 0; r < boardSize; r++) {
        for (int c = 0; c < boardSize; c++) {
            if (boardState[r][c] == 0) {
                boardState[r][c] = 2;
                if (checkWin(2)) { boardState[r][c] = 0; return new int[]{r, c}; }
                boardState[r][c] = 0;
            }
        }
    }

    for (int r = 0; r < boardSize; r++) {
        for (int c = 0; c < boardSize; c++) {
            if (boardState[r][c] == 0) {
                boardState[r][c] = 1;
                if (checkWin(1)) { boardState[r][c] = 0; return new int[]{r, c}; }
                boardState[r][c] = 0;
            }
        }
    }

    int bestVal = -1000000;
    int[] bestMove = {-1, -1};
    for (int r = 0; r < boardSize; r++) {
        for (int c = 0; c < boardSize; c++) {
            if (boardState[r][c] == 0) {
                boardState[r][c] = 2;
                int moveVal = minimax(0, false, -1000000, 1000000, 3);
                boardState[r][c] = 0;
                if (moveVal > bestVal) {
                    bestVal = moveVal;
                    bestMove[0] = r; bestMove[1] = c;
                }
            }
        }
    }
    return bestMove;
}

private int minimax(int depth, boolean isMax, int alpha, int beta, int maxDepth) {
    if (checkWin(2)) return 10000 - depth;
    if (checkWin(1)) return depth - 10000;
    if (isBoardFull() || depth >= maxDepth) return evaluateBoard();

    if (isMax) {
        int best = -1000000;
        for (int r = 0; r < boardSize; r++) {
            for (int c = 0; c < boardSize; c++) {
                if (boardState[r][c] == 0) {
                    boardState[r][c] = 2;
                    best = Math.max(best, minimax(depth + 1, false, alpha, beta, maxDepth));
                    boardState[r][c] = 0;
                    alpha = Math.max(alpha, best);
                    if (beta <= alpha) break;
                }
            }
        }
        return best;
    } else {
        int best = 1000000;
        for (int r = 0; r < boardSize; r++) {
            for (int c = 0; c < boardSize; c++) {
                if (boardState[r][c] == 0) {
                    boardState[r][c] = 1;
                    best = Math.min(best, minimax(depth + 1, true, alpha, beta, maxDepth));
                    boardState[r][c] = 0;
                    beta = Math.min(beta, best);
                    if (beta <= alpha) break;
                }
            }
        }
        return best;
    }
}

private int evaluateBoard() {
    int score = 0;
    int[][] dirs = {{0, 1}, {1, 0}, {1, 1}, {1, -1}};

    for (int r = 0; r < boardSize; r++) {
        for (int c = 0; c < boardSize; c++) {
            if (boardState[r][c] == 0) continue;
            
            int player = boardState[r][c];
            int m = (player == 2) ? 1 : -1;

            for (int[] d : dirs) {
                int count = 0;
                for (int i = 0; i < winCondition; i++) {
                    int nr = r + d[0] * i, nc = c + d[1] * i;
                    if (nr >= 0 && nr < boardSize && nc >= 0 && nc < boardSize && boardState[nr][nc] == player) count++;
                    else break;
                }
                
                if (count >= winCondition) score += (player == 2 ? 100000 : -500000) * m;
                else if (count == 3) score += (player == 2 ? 5000 : -20000) * m; 
                else if (count == 2) score += (player == 2 ? 200 : -1000) * m;
            }
        }
    }
    return score;
}
}