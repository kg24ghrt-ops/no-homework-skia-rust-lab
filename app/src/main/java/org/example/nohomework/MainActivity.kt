package org.example.nohomework

import android.os.Bundle
import android.widget.Button
import android.widget.LinearLayout
import android.widget.TextView
import androidx.appcompat.app.AppCompatActivity

class MainActivity : AppCompatActivity() {

    init {
        System.loadLibrary("no_homework_native")
    }

    external fun greeting(): String
    external fun invertColors(width: Int, height: Int, inputBytes: ByteArray): ByteArray

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        val layout = LinearLayout(this)
        layout.orientation = LinearLayout.VERTICAL
        layout.setPadding(24, 24, 24, 24)

        val greetingText = TextView(this)
        greetingText.text = "Greeting: $greeting()"
        greetingText.textSize = 18f

        val resultText = TextView(this)
        resultText.text = "Invert colors: not tested yet"
        resultText.textSize = 16f

        val testButton = Button(this)
        testButton.text = "Test invertColors (2x2 RGBA)"

        testButton.setOnClickListener {
            // 2x2 RGBA8 pixels: [R,G,B,A] x 4
            val input = byteArrayOf(
                10, 20, 30, 200,  // pixel 0
                40, 50, 60, 100,  // pixel 1
                70, 80, 90, 50,   // pixel 2
                10, 20, 30, 255   // pixel 3
            )
            val output = invertColors(2, 2, input)
            val hex = output.joinToString("") { "%02x".format(it) }
            resultText.text = "Invert colors (2x2): $hex"
        }

        layout.addView(greetingText)
        layout.addView(testButton)
        layout.addView(resultText)

        setContentView(layout)
    }
}
