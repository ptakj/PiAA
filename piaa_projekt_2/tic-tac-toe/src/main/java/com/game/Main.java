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

        stage.setTitle("TEST123");
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
    System.err.println("DEBUG: Buduję planszę o rozmiarze: " + this.boardSize);

     if (this.gamaplayInterfaceView != null) {
            return this.gamaplayInterfaceView;
        }


    GridPane grid = new GridPane();
    grid.setAlignment(Pos.CENTER);
    grid.setHgap(5);
    grid.setVgap(5);

    for (int row = 0; row < this.boardSize; row++) {
        for (int col = 0; col < this.boardSize; col++) {
            Button btn = new Button(row + "," + col);
            btn.setPrefSize(60, 60); 
            

            //final int r = row;
            //final int c = col;
            
            //btn.setOnAction(e -> handleCellClick(r, c, btn));
            
            grid.add(btn, col, row); 
        }
    }
    this.gamaplayInterfaceView = grid;
    return this.gamaplayInterfaceView;
    }

}

 