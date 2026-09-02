import 'package:flutter_test/flutter_test.dart';
import 'package:document_converter/main.dart';
void main(){testWidgets('中文首页显示', (tester) async {await tester.pumpWidget(const App());expect(find.text('离线文档转换器'), findsOneWidget);expect(find.text('选择文件（可多选）'), findsOneWidget);});}
