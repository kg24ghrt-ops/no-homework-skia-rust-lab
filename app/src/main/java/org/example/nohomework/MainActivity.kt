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
    external fun generateNotebookPaper(
        width: Int,
        height: Int,
        lineSpacing: Float,
        marginWidth: Float,
    ): ByteArray

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        val layout = LinearLayout(this)
        layout.orientation = LinearLayout.VERTICAL
        layout.setPadding(24, 24, 24, 24)

        val greetingText = TextView(this)
        greetingText.text = "Greeting: ${this.greeting()}"
        greetingText.textSize = 18f

        val invertResult = TextView(this)
        invertResult.text = "Invert colors: not tested yet"
        invertResult.textSize = 16f

        val invertButton = Button(this)
        invertButton.text = "Test invertColors (2x2 RGBA)"

        val paperResult = TextView(this)
        paperResult.text = "Notebook paper (B5): not generated yet"
        paperResult.textSize = 16f

        val paperButton = Button(this)
        paperButton.text = "Generate B5 Notebook Paper"

        invertButton.setOnClickListener {
            val input = byteArrayOf(
                10.toByte(), 20.toByte(), 30.toByte(), 200.toByte(),
                40.toByte(), 50.toByte(), 60.toByte(), 100.toByte(),
                70.toByte(), 80.toByte(), 90.toByte(), 50.toByte(),
                10.toByte(), 20.toByte(), 30.toByte(), 255.toByte(),
            )
            val output = invertColors(2, 2, input)
            val hex = output.joinToString("") { "%02x".format(it) }
            invertResult.text = "Invert colors (2x2): $hex"
        }

        // B5 at 150 DPI: 176mm x 250mm -> approx 1039 x 1476 px
        paperButton.setOnClickListener {
            val dims = b5DimensionsAtDpi(150f)
            val png = generateNotebookPaper(dims.width, dims.height, 30.0, 40.0)
            val hex = png.take(16).joinToString("") { "%02x".format(it) }
            val isPng = png.size >= 8 &&
                png[0] == 0x89.toByte() && png[1] == 0x50.toByte() &&
                png[2] == 0x4e.toByte() && png[3] == 0x47.toByte()
            paperResult.text =
                "Notebook paper (B5 ${dims.width}x${dims.height}): ${png.size} bytes, PNG=$isPng, head=$hex"
        }

        layout.addView(greetingText)
        layout.addView(invertButton)
        layout.addView(invertResult)
        layout.addView(paperButton)
        layout.addView(paperResult)

        setContentView(layout)
    }

    /** B5 paper dimensions at a given DPI (176mm x 250mm, common in Asia). */
    private fun b5DimensionsAtDpi(dpi: Float): Pair<Int, Int> {
        val mmToPx = { mm: Float -> (mm / 25.4f * dpi).round().toInt() }
        return Pair(mmToPx(176f), mmToPx(250f))
    }
}
