# 模拟磅机

## 磅机分类

1. D2008
2. A9

### 模拟程序

> 模拟程序运行与 Arduino 程序下

``` javascript
int counter = 0;
void setup() {
  // put your setup code here, to run once:
  Serial.begin(9600);
}
//  ACSii
void loop() {

  for(counter = 0;counter <= 50;counter++) {
//  起始位
    Serial.write(2);
//  符号位
    Serial.write(43);
//  重量
    Serial.write(48);
    Serial.write(48);
    Serial.write(48);
    Serial.write(48);
    Serial.write(48);
    Serial.write(48);
//  小数点位数
    Serial.write(50);
    Serial.write(49);
    Serial.write(11);
    Serial.write(3);
    delay(200);
  }
  for(counter = 0;counter <= 50;counter++) {
//  起始位
    Serial.write(2);
//  符号位
    Serial.write(43);
//  重量
    Serial.write(48);
    Serial.write(48);
    Serial.write(49);
    Serial.write(51);
    Serial.write(52);
    Serial.write(48);
//  小数点位数
    Serial.write(50);
    Serial.write(49);
    Serial.write(11);
    Serial.write(3);
    delay(200);
  }
  for(counter = 0;counter <= 50;counter++) {
//  起始位
    Serial.write(2);
//  符号位
    Serial.write(43);
//  重量
    Serial.write(48);
    Serial.write(48);
    Serial.write(48);
    Serial.write(48);
    Serial.write(48);
    Serial.write(48);
//  小数点位数
    Serial.write(50);
    Serial.write(49);
    Serial.write(11);
    Serial.write(3);
    delay(200);
  }
  for(counter = 0;counter <= 50;counter++) {
//  起始位
    Serial.write(2);
//  符号位
    Serial.write(43);
//  重量
    Serial.write(48);
    Serial.write(48);
    Serial.write(52);
    Serial.write(56);
    Serial.write(57);
    Serial.write(48);
//  小数点位数
    Serial.write(50);
    Serial.write(49);
    Serial.write(11);
    Serial.write(3);
    delay(200);
  }
}
```

## 附件
Arduino [官网](https://www.arduino.cc/en/software)