
import 'package:flutter/material.dart';

class THelperFunctions {

  static bool isDarkMode(BuildContext context) {
    return Theme.of(context).brightness == Brightness.dark;
  }
}