import 'package:flutter/material.dart';
import '../../../../utils/constants/sizes.dart';

class TSpacingStyles {
  TSpacingStyles._();

  static const EdgeInsetsGeometry defaultPaddingWithAppBarHeight = EdgeInsets.only(
    top: TSizes.appBarHeight,
    left: TSizes.defaultSpace,
    bottom: TSizes.defaultSpace,
    right: TSizes.defaultSpace,
  );
}