package com.game;

import javafx.scene.paint.Paint;

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
        launch(args);
    }

    public void start(Stage stage) throws Exception {
        Ui ui = new Ui();

        stage.setTitle("Game Start");
        stage.setScene(ui.GetScene());
        stage.show();
    }

    public void stop() throws Exception {
        System.out.println("stopping");
    }
}

class Ui {
    Scene scene = null;
    Node gameStartView = null;
    Node gamaplayInterfaceView = null;

    GameMode state = GameMode.GAME_START;

    public Ui() {
        this.scene = new Scene(new Group());
        this.changeScene();
    }

    public Scene GetScene() {
        return this.scene;
    }

    public void changeScene() {
        ObservableList<Node> rootList = ((Group)scene.getRoot()).getChildren();

        switch (state) {
            case GAME_START:
                rootList.clear();
                rootList.addAll(this.getGameStartView());    
                break;
            default:
                rootList.clear();
                //rootList.addAll(this.getGameOnView());
                break;
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
            if(boardSizeSlider.getValue() >= winSizeSlider.getValue()){
                if(checkBx1.isSelected() == false)
                    this.state = GameMode.PVP;
                else
                    this.state = GameMode.PVE;
                
            this.changeScene();
             
            }
            else{
                System.out.println("Cannot start game with impossible win condition!");
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

   public Node getgameplayInterfaceView(){
     if (this.gamaplayInterfaceView != null) {
            return this.gamaplayInterfaceView;
        }


        return this.gamaplayInterfaceView;
    }

}

 