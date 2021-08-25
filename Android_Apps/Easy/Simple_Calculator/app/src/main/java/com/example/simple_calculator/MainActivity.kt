package com.example.simple_calculator

import android.os.Bundle
import android.util.Log
import android.view.View
import android.widget.Button
import android.widget.EditText
import android.widget.TextView
import androidx.appcompat.app.AppCompatActivity
import com.example.simple_calculator.databinding.ActivityMainBinding
import kotlin.math.pow

class MainActivity : AppCompatActivity(), View.OnClickListener {
    private lateinit var binding: ActivityMainBinding

    private lateinit var num1View: EditText
    private lateinit var num2View: EditText
    private lateinit var result : TextView

    private fun getNumbers(v1: View = num1View, v2: View = num2View, resultTv: TextView = result ): Pair<Int, Int> {
        val num1: Int = v1.text.toInt()
        val num2: Int = v2.text.toString().toInt()

        if (num1.equals(null) && num2.equals(null)) {
            resultTv.text = getString(R.string.inputNull)
        }
        return Pair(num1, num2)
    }

    private fun doSum(num1: Int, num2: Int): String {
        val sum: Int = num1 + num2
        return (sum.toString())
    }

    private fun doPow(base: Int, exponent: Int): String {
        val pow = base.toDouble().pow(exponent.toDouble())
        return (pow.toString())
    }

    private fun doMod(num1: Int, num2: Int): String {
        val mod = num1 % num2
        return (mod.toString())
    }

    private fun doDiv(num1: Int, num2: Int): String {
        val div = num1 / num2
        return (div.toString())
    }

    private fun doMul(num1: Int, num2: Int): String {
        val mul = num1 * num2
        return (mul.toString())
    }
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        binding = ActivityMainBinding.inflate(layoutInflater)
        setContentView(binding.root)

        val div: Button = binding.div
        div.setOnClickListener(this)
        val mul: Button = binding.mul
        mul.setOnClickListener(this)
        val sum: Button = binding.sum
        sum.setOnClickListener(this)
        val pow: Button = binding.pow
        pow.setOnClickListener(this)
        val mod: Button = binding.mod
        mod.setOnClickListener(this)


    }


    override fun onClick(p0: View?) {
        num1View = binding.num1
        num2View = binding.num2
        result = binding.result

        Log.d("debug", "ReachedOnClick")


        if (p0 != null) {
            when (p0.id) {
                binding.sum.id -> {
                    val (num1, num2) = getNumbers(num1View, num2View, result)
                    Log.d("debug", "Reached when conditional")
                    result.text = doSum(num1, num2)
                }
                binding.div.id -> {
                    val (num1, num2) = getNumbers(num1View, num2View, result)
                    result.text = doDiv(num1, num2)
                }
                binding.mod.id -> {
                    val (num1, num2) = getNumbers(num1View, num2View, result)
                    result.text = doMod(num1, num2)
                }
                binding.pow.id -> {
                    val (num1, num2) = getNumbers(num1View, num2View, result)
                    result.text = doPow(num1, num2)
                }
                binding.mul.id -> {
                    val (num1, num2) = getNumbers(num1View, num2View, result)
                    result.text = doMul(num1, num2)
                }
            }
        }
    }

}