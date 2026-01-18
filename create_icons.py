from PIL import Image, ImageDraw, ImageFont


def create_icon(size, text):
    img = Image.new("RGBA", (size, size), (255, 215, 0, 255))
    draw = ImageDraw.Draw(img)

    # 画边框
    draw.rectangle([0, 0, size - 1, size - 1], outline=(255, 165, 0), width=2)

    # 画文字
    try:
        font = ImageFont.truetype("arial.ttf", size // 2)
    except:
        font = ImageFont.load_default()

    bbox = draw.textbbox((0, 0), text, font=font)
    text_width = bbox[2] - bbox[0]
    text_height = bbox[3] - bbox[1]

    x = (size - text_width) // 2
    y = (size - text_height) // 2

    draw.text((x, y), text, font=font, fill=(139, 69, 19))

    return img


# 创建32x32图标
img32 = create_icon(32, "象")
img32.save("icons/32x32.png")

# 创建128x128图标
img128 = create_icon(128, "象")
img128.save("icons/128x128.png")

print("Icons created successfully!")
