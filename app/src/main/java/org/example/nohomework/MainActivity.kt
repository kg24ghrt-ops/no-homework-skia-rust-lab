package org.example.nohomework

import android.os.Bundle
import android.widget.TextView
import androidx.appcompat.app.AppCompatActivity

class MainActivity : AppCompatActivity() {

    init {
        System.loadLibrary("no_homework_native")
    }

    external fun greeting(): String
    // external fun invertColors(width: Int, height: Int, inputBytes: ByteArray): ByteArray

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        val textView = TextView(this)
        textView.text = greeting()
        setContentView(textView)
    }
}
