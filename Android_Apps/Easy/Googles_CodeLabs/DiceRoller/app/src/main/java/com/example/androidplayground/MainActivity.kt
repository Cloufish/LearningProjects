package com.example.androidplayground

import android.os.Bundle
import android.widget.Button
import android.widget.ImageView
import android.widget.Toast
import androidx.appcompat.app.AppCompatActivity

class MainActivity : AppCompatActivity() {

    private lateinit var diceImage1 : ImageView
    private lateinit var diceImage2 : ImageView

    override fun onCreate(savedInstanceState: Bundle?) {

        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        val rollButton = findViewById<Button>(R.id.changeTextBtn)
        diceImage1 = findViewById(R.id.diceImage1)
        diceImage2 = findViewById(R.id.diceImage2)


        rollButton.setOnClickListener { rollDice() }
    }

        private fun randomDice() : Int{
            val randomNumber = (1..6).random()

            return when (randomNumber){
                1 -> R.drawable.dice_1
                2 -> R.drawable.dice_2
                3 -> R.drawable.dice_3
                4 -> R.drawable.dice_4
                5 -> R.drawable.dice_5
                else -> R.drawable.dice_6
            }
        }

        private fun rollDice() {


            Toast.makeText(this, "Rolled Dice",
                Toast.LENGTH_SHORT).show()

            diceImage1.setImageResource(randomDice())
            diceImage2.setImageResource(randomDice())



        }
}
