GDPC                                                                               <   res://.import/icon.png-487276ed1e3a0c39cad0279d744ee560.stex�!      �      &�y���ڞu;>��.p   res://Backend/Backend.gdns         �       ���(�yՕ���Ms   res://Backend/Backend.tscn         9      �rKC��z.�Ҙ$   res://Backend/BackendHandle.gd.remap�0      0       D��1Nm*��M�    res://Backend/BackendHandle.gdc @      �
      I�~_�.�?��줉@�   res://Bin/lib.gdnlib       �       d�<��]��=O��$   res://Character/Character.gd.remap  �0      .       45S��b4��*�;���    res://Character/Character.gdc   �      ,      ��j��|u1Q��4"ڭ    res://Character/Character.tscn  �      ~      6��ΣUC���?!<��   res://Cube.tscn p       �       �\`y!*$�%��3��_   res://default_env.tres  !      �       um�`�N��<*ỳ�8   res://icon.png  �0      �      G1?��z�c��vN��   res://icon.png.import   �'      �      ��fe��6�B��^ U�   res://main.tscn P*      /      D��՚n���;�����   res://project.binary�=      r      Őцq
��,�)苙��        [gd_resource type="NativeScript" load_steps=2 format=2]

[ext_resource path="res://Bin/lib.gdnlib" type="GDNativeLibrary" id=1]

[resource]
resource_name = "Backend"
class_name = "Backend"
library = ExtResource( 1 )
        [gd_scene load_steps=3 format=2]

[ext_resource path="res://Backend/Backend.gdns" type="Script" id=1]
[ext_resource path="res://Backend/BackendHandle.gd" type="Script" id=2]

[node name="BackendHandle" type="Node"]
script = ExtResource( 2 )

[node name="Backend" type="Node" parent="."]
script = ExtResource( 1 )
       GDSC   -      M   �     ���Ӷ���   ���¶���   �����¶�   �������Ӷ���   ����ض��   ����������������   �����϶�   �������������������ض���   ���Ӷ���   ���Ӷ���   ����   ����������Ŷ   ���ض���   ���򶶶�   ���������Ķ�   ����Ӷ��   �������Ķ���   ����ض��   �������Ŷ���   ��¶   ����¶��   ����������¶   ���Ӷ���   ����󶶶   �����������Ķ���   ����Ķ��   ��Ķ   ��������϶��   �����Ӷ�   ������������������Ӷ   ����ݶ��   ����������ڶ   ���������������������Ҷ�   ���Ӷ���   �����Ѷ�   ���Ҷ���   ����϶��   �����������Ѷ���   ������Ӷ   ���Ӷ���   ��Ŷ   ζ��   �����������Ŷ���   ϶��   ̶��      0.0.0.0:8000      Backend             user://token.dat      Luca             Token               Error         Login         invalid login token       Register      user already registered       ChunkUpdate       {"Register":{"name":"         "}}                        {"Login":{"token":        }}     
         {"Move":{"coord":[        ,         ]}}                                            &      '   	   /   
   0      6      >      ?      @      J      S      _      i      p      s      {      |      �      �      �      �      �      �      �      �      �       �   !   �   "   �   #   �   $   �   %   �   &   �   '   �   (   �   )   �   *   �   +   �   ,   �   -     .     /     0     1     2     3     4      5   '  6   1  7   2  8   :  9   B  :   C  ;   L  <   V  =   ^  >   _  ?   e  @   n  A   �  B   �  C   �  D   �  E   �  F   �  G   �  H   �  I   �  J   �  K   �  L   �  M   3SYY8;�  V�  SYY5;�  VT�  P�  QSYY;�  V�  LMSYY:�  V�  �  SYY0�  PQV�  �  T�  P�  QS�  �  �  ;�  �	  T�
  PQS�  &�  T�  P�  QV�  �  T�  P�  R�	  T�  QS�  �  �  T�  P�  QS�  �  T�  PQS�  (V�  T�  P�  QS�  �  T�  PQS�  Y0�  P�  V�  QV�  �  ;�  �  T�  PQS�  &�  V�  ;�  �  L�  MS�  /�  V�  �  V�  �  �  �  L�  ML�  MS�  T�  PQS�  �  ;�  �	  T�
  PQS�  �  T�  P�  R�	  T�  QS�  �  T�  P�  QS�  �  T�  PQSY�  �  V�  ;�  �  L�  ML�  MS�  /�  V�  �	  V�  �?  P�
  QS�  ;�  �  T�
  PQS�  �  T�  P�  QS�  �  V�  �?  P�  QS�  �  YY0�  P�  QV�  T�  P�  R�  QS�  Y0�   PQX�  V�  .�  T�   PQS�  Y0�  P�!  V�  QV�  ;�"  �  �!  �  S�  �  T�#  P�"  QS�  Y0�  PQV�  &�  T�$  PQV�  ;�%  V�  �>  P�  QT�&  P�  R�  QS�  ;�"  �  �%  �  S�  �  T�#  P�"  QS�  YY0�'  P�(  V�  QV�  ;�)  �>  P�(  T�)  QT�*  P�  QS�  ;�+  �>  P�(  T�+  QT�*  P�  QS�  ;�,  �>  P�(  T�,  QT�*  P�  QS�  ;�"  �  �)  �  �+  �  �,  �  S�  �  �  T�#  P�"  QSY`            [general]

singleton=false
load_once=true
symbol_prefix="godot_"
reloadable=true

[entry]

X11.64="res://Bin/librust.so"

[dependencies]

X11.64=[  ]
          GDSC   0      V   >     ������������϶��   ��������Ķ��   �������Ӷ���   ������Ҷ   ����򶶶   �������   ������������   ���涶��   �������������������   �������϶���   ��������   �����Ķ�   �������������Ҷ�   ���������Ŷ�   ���������������ζ���   ���������������϶���   �����¶�   ����¶��   ��������������������ض��   ���������������Ŷ���   ϶��   �������Ӷ���   ζ��   �����׶�   �����������¶���   ��¶   ��������Ķ��   ����¶��   ����������������Ҷ��   ���������������۶���   ����Ŷ��   ̶��   ���������������������Ҷ�   �������������Ӷ�   ������������������   �������������������   ��ƶ   ��̶   ���������Ҷ�   ���������������Ŷ���   �¶�   ����Ӷ��   �������������Ӷ�   �涶   ����������Ķ   ���Ӷ���   ��������۶��   �����ض�      ../Character      ../../BackendHandle      �?      A     �?  ��Q��?                             Z      2      
         move_forward      move_backwards     	   move_left      
   move_right        jump   	   ui_cancel         window_click      ?                                            !      *      3   	   <   
   E      F      P      X      Y      a      f      k      p      q      r      y      �      �      �      �      �      �      �      �      �      �       �   !   �   "   �   #   �   $     %     &     '   $  (   0  )   9  *   D  +   E  ,   F  -   R  .   Y  /   ^  0   _  1   `  2   i  3   s  4   x  5   �  6   �  7   �  8   �  9   �  :   �  ;   �  <   �  =   �  >   �  ?   �  @   �  A   �  B   �  C   �  D   �  E   �  F   �  G   �  H   �  I   �  J   �  K   
  L     M     N     O     P      Q   '  R   .  S   /  T   0  U   <  V   3YY5;�  �  PQSY5;�  �  P�  QSYY8;�  V�  �  SY8;�  V�  �  SY8;�  V�  �  SY8;�  V�  �  SY8;�  V�  �  SYY;�	  V�  �  T�
  SY;�  V�  �  SYY;�  V�  �  SY;�  �  Y;�  �  Y;�  �  YYY0�  P�  QV�  &�  4�  �  V�  T�  T�  �  T�  T�  �  �	  �  W�  T�  T�  �  T�  T�  �  �	  �  W�  T�  T�  �5  PW�  T�  T�  R�
  R�
  Q�  �  �  �5  P�  T�  T�  R�  R�  Q�  �  �5  P�  T�  T�  R�  R�  QYYYY0�  P�  V�  QX�  V�  ;�  V�  �  T�
  S�  �  &�  T�  P�  QV�  �  T�  T�  T�  S�  &�  T�  P�  QV�  �  T�  T�  T�  S�  &�  T�  P�  QV�  �  T�  T�  T�  S�  &�  T�  P�  QV�  �  T�  T�  T�  S�  �  �  &�  �  T�   P�  QV�  �  T�  �  S�  �  �  S�  �  �  &�  T�  P�  QV�  �  T�!  P�  T�"  QS�  �  �  S�  &�  T�  P�  QV�  �  T�!  P�  T�#  QS�  �  �  S�  �  �  �  ;�$  �  S�  &�  V�  �$  �  S�  �  ;�%  V�  �  P�  T�  R�  T�  QT�&  PQ�$  �  S�  �  .�  P�%  T�  R�  T�  R�%  T�  QSYY0�'  P�(  V�  QV�  ;�  �  P�(  QS�  �	  �  S�  �  �  &�	  T�  �  V�  �	  T�  �  S�  �  ;�)  T�*  P�	  R�  T�+  R�  QS�  &T�,  PQV�  �	  T�  �  S�  �  �  S�  �  �  �	  T�  �  S�  �	  T�  �  S�  �  �  �  T�-  PT�.  T�/  QSY`    [gd_scene load_steps=3 format=2]

[ext_resource path="res://Character/Character.gd" type="Script" id=1]

[sub_resource type="CapsuleShape" id=1]
radius = 5.0
height = 2.0

[node name="Character" type="KinematicBody"]
script = ExtResource( 1 )
GRAVITY = 15.0
JUMP = 12.0

[node name="CollisionShape" type="CollisionShape" parent="."]
pause_mode = 1
process_priority = 10
transform = Transform( -4.37114e-08, 1, -4.37114e-08, 0, -4.37114e-08, -1, -1, -4.37114e-08, 1.91069e-15, 0, 0, 0 )
shape = SubResource( 1 )

[node name="Camera" type="Camera" parent="."]
transform = Transform( 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0.772764, 0 )
far = 2500.0
  [gd_scene load_steps=2 format=2]

[sub_resource type="CubeMesh" id=1]

[node name="MeshInstance" type="MeshInstance"]
mesh = SubResource( 1 )
material/0 = null
[gd_resource type="Environment" load_steps=2 format=2]

[sub_resource type="ProceduralSky" id=1]

[resource]
background_mode = 2
background_sky = SubResource( 1 )
             GDST@   @            �  WEBPRIFF�  WEBPVP8L�  /?����m��������_"�0@��^�"�v��s�}� �W��<f��Yn#I������wO���M`ҋ���N��m:�
��{-�4b7DԧQ��A �B�P��*B��v��
Q�-����^R�D���!(����T�B�*�*���%E["��M�\͆B�@�U$R�l)���{�B���@%P����g*Ųs�TP��a��dD
�6�9�UR�s����1ʲ�X�!�Ha�ߛ�$��N����i�a΁}c Rm��1��Q�c���fdB�5������J˚>>���s1��}����>����Y��?�TEDױ���s���\�T���4D����]ׯ�(aD��Ѓ!�a'\�G(��$+c$�|'�>����/B��c�v��_oH���9(l�fH������8��vV�m�^�|�m۶m�����q���k2�='���:_>��������á����-wӷU�x�˹�fa���������ӭ�M���SƷ7������|��v��v���m�d���ŝ,��L��Y��ݛ�X�\֣� ���{�#3���
�6������t`�
��t�4O��ǎ%����u[B�����O̲H��o߾��$���f���� �H��\��� �kߡ}�~$�f���N\�[�=�'��Nr:a���si����(9Lΰ���=����q-��W��LL%ɩ	��V����R)�=jM����d`�ԙHT�c���'ʦI��DD�R��C׶�&����|t Sw�|WV&�^��bt5WW,v�Ş�qf���+���Jf�t�s�-BG�t�"&�Ɗ����׵�Ջ�KL�2)gD� ���� NEƋ�R;k?.{L�$�y���{'��`��ٟ��i��{z�5��i������c���Z^�
h�+U�mC��b��J��uE�c�����h��}{�����i�'�9r�����ߨ򅿿��hR�Mt�Rb���C�DI��iZ�6i"�DN�3���J�zڷ#oL����Q �W��D@!'��;�� D*�K�J�%"�0�����pZԉO�A��b%�l�#��$A�W�A�*^i�$�%a��rvU5A�ɺ�'a<��&�DQ��r6ƈZC_B)�N�N(�����(z��y�&H�ض^��1Z4*,RQjԫ׶c����yq��4���?�R�����0�6f2Il9j��ZK�4���է�0؍è�ӈ�Uq�3�=[vQ�d$���±eϘA�����R�^��=%:�G�v��)�ǖ/��RcO���z .�ߺ��S&Q����o,X�`�����|��s�<3Z��lns'���vw���Y��>V����G�nuk:��5�U.�v��|����W���Z���4�@U3U�������|�r�?;�
         [remap]

importer="texture"
type="StreamTexture"
path="res://.import/icon.png-487276ed1e3a0c39cad0279d744ee560.stex"
metadata={
"vram_texture": false
}

[deps]

source_file="res://icon.png"
dest_files=[ "res://.import/icon.png-487276ed1e3a0c39cad0279d744ee560.stex" ]

[params]

compress/mode=0
compress/lossy_quality=0.7
compress/hdr_mode=0
compress/bptc_ldr=0
compress/normal_map=0
flags/repeat=0
flags/filter=true
flags/mipmaps=false
flags/anisotropic=false
flags/srgb=2
process/fix_alpha_border=true
process/premult_alpha=false
process/HDR_as_SRGB=false
process/invert_color=false
process/normal_map_invert_y=false
stream=false
size_limit=0
detect_3d=true
svg/scale=1.0
              [gd_scene load_steps=6 format=2]

[ext_resource path="res://Character/Character.tscn" type="PackedScene" id=1]
[ext_resource path="res://Backend/Backend.tscn" type="PackedScene" id=3]

[sub_resource type="ProceduralSky" id=4]

[sub_resource type="Environment" id=3]
background_mode = 3
background_sky = SubResource( 4 )
background_color = Color( 0.34902, 0.560784, 0.784314, 1 )
ambient_light_color = Color( 1, 0.933333, 0.635294, 1 )
ambient_light_energy = 0.25
ambient_light_sky_contribution = 0.25
fog_enabled = true
fog_color = Color( 0.34902, 0.560784, 0.784314, 1 )
fog_sun_amount = 1.0
fog_depth_begin = 400.0
fog_depth_end = 466.0
ssao_enabled = true
ssao_quality = 2
dof_blur_far_distance = 200.0
dof_blur_far_transition = 100.0
dof_blur_far_amount = 0.03
dof_blur_far_quality = 2
glow_bloom = 1.0

[sub_resource type="BoxShape" id=2]
extents = Vector3( 1, 0.307304, 0.85674 )

[node name="WorldEnvironment" type="WorldEnvironment"]
environment = SubResource( 3 )

[node name="main" type="Spatial" parent="."]

[node name="Character" parent="main" instance=ExtResource( 1 )]
transform = Transform( 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 76.1999, -0.170842 )
SPEED = 4.0
GRAVITY = 40.0
JUMP = 20.0
IN_AIR_MOVEMENT_CAP = 1.0

[node name="StaticBody" type="StaticBody" parent="main"]
transform = Transform( 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 67.6699, 0 )

[node name="CollisionShape" type="CollisionShape" parent="main/StaticBody"]
transform = Transform( 2, 0, 0, 0, 1, 0, 0, 0, 1.915, 0, 6.384, 0 )
shape = SubResource( 2 )

[node name="BackendHandle" parent="." instance=ExtResource( 3 )]
 [remap]

path="res://Backend/BackendHandle.gdc"
[remap]

path="res://Character/Character.gdc"
  �PNG

   IHDR   @   @   �iq�   sRGB ���  �IDATx��ytTU��?�ի%���@ȞY1JZ �iA�i�[P��e��c;�.`Ow+4�>�(}z�EF�Dm�:�h��IHHB�BR!{%�Zߛ?��	U�T�
���:��]~�������-�	Ì�{q*�h$e-
�)��'�d�b(��.�B�6��J�ĩ=;���Cv�j��E~Z��+��CQ�AA�����;�.�	�^P	���ARkUjQ�b�,#;�8�6��P~,� �0�h%*QzE� �"��T��
�=1p:lX�Pd�Y���(:g����kZx ��A���띊3G�Di� !�6����A҆ @�$JkD�$��/�nYE��< Q���<]V�5O!���>2<��f��8�I��8��f:a�|+�/�l9�DEp�-�t]9)C�o��M~�k��tw�r������w��|r�Ξ�	�S�)^� ��c�eg$�vE17ϟ�(�|���Ѧ*����
����^���uD�̴D����h�����R��O�bv�Y����j^�SN֝
������PP���������Y>����&�P��.3+�$��ݷ�����{n����_5c�99�fbסF&�k�mv���bN�T���F���A�9�
(.�'*"��[��c�{ԛmNު8���3�~V� az
�沵�f�sD��&+[���ke3o>r��������T�]����* ���f�~nX�Ȉ���w+�G���F�,U�� D�Դ0赍�!�B�q�c�(
ܱ��f�yT�:��1�� +����C|��-�T��D�M��\|�K�j��<yJ, ����n��1.FZ�d$I0݀8]��Jn_� ���j~����ցV���������1@M�)`F�BM����^x�>
����`��I�˿��wΛ	����W[�����v��E�����u��~��{R�(����3���������y����C��!��nHe�T�Z�����K�P`ǁF´�nH啝���=>id,�>�GW-糓F������m<P8�{o[D����w�Q��=N}�!+�����-�<{[���������w�u�L�����4�����Uc�s��F�륟��c�g�u�s��N��lu���}ן($D��ת8m�Q�V	l�;��(��ڌ���k�
s\��JDIͦOzp��مh����T���IDI���W�Iǧ�X���g��O��a�\:���>����g���%|����i)	�v��]u.�^�:Gk��i)	>��T@k{'	=�������@a�$zZ�;}�󩀒��T�6�Xq&1aWO�,&L�cřT�4P���g[�
p�2��~;� ��Ҭ�29�xri� ��?��)��_��@s[��^�ܴhnɝ4&'
��NanZ4��^Js[ǘ��2���x?Oܷ�$��3�$r����Q��1@�����~��Y�Qܑ�Hjl(}�v�4vSr�iT�1���f������(���A�ᥕ�$� X,�3'�0s����×ƺk~2~'�[�ё�&F�8{2O�y�n�-`^/FPB�?.�N�AO]]�� �n]β[�SR�kN%;>�k��5������]8������=p����Ցh������`}�
�J�8-��ʺ����� �fl˫[8�?E9q�2&������p��<�r�8x� [^݂��2�X��z�V+7N����V@j�A����hl��/+/'5�3�?;9
�(�Ef'Gyҍ���̣�h4RSS� ����������j�Z��jI��x��dE-y�a�X�/�����:��� +k�� �"˖/���+`��],[��UVV4u��P �˻�AA`��)*ZB\\��9lܸ�]{N��礑]6�Hnnqqq-a��Qxy�7�`=8A�Sm&�Q�����u�0hsPz����yJt�[�>�/ޫ�il�����.��ǳ���9��
_
��<s���wT�S������;F����-{k�����T�Z^���z�!t�۰؝^�^*���؝c
���;��7]h^
��PA��+@��gA*+�K��ˌ�)S�1��(Ե��ǯ�h����õ�M�`��p�cC�T")�z�j�w��V��@��D��N�^M\����m�zY��C�Ҙ�I����N�Ϭ��{�9�)����o���C���h�����ʆ.��׏(�ҫ���@�Tf%yZt���wg�4s�]f�q뗣�ǆi�l�⵲3t��I���O��v;Z�g��l��l��kAJѩU^wj�(��������{���)�9�T���KrE�V!�D���aw���x[�I��tZ�0Y �%E�͹���n�G�P�"5FӨ��M�K�!>R���$�.x����h=gϝ�K&@-F��=}�=�����5���s �CFwa���8��u?_����D#���x:R!5&��_�]���*�O��;�)Ȉ�@�g�����ou�Q�v���J�G�6�P�������7��-���	պ^#�C�S��[]3��1���IY��.Ȉ!6\K�:��?9�Ev��S]�l;��?/� ��5�p�X��f�1�;5�S�ye��Ƅ���,Da�>�� O.�AJL(���pL�C5ij޿hBƾ���ڎ�)s��9$D�p���I��e�,ə�+;?�t��v�p�-��&����	V���x���yuo-G&8->�xt�t������Rv��Y�4ZnT�4P]�HA�4�a�T�ǅ1`u\�,���hZ����S������o翿���{�릨ZRq��Y��fat�[����[z9��4�U�V��Anb$Kg������]������8�M0(WeU�H�\n_��¹�C�F�F�}����8d�N��.��]���u�,%Z�F-���E�'����q�L�\������=H�W'�L{�BP0Z���Y�̞���DE��I�N7���c��S���7�Xm�/`�	�+`����X_��KI��^��F\�aD�����~�+M����ㅤ��	SY��/�.�`���:�9Q�c �38K�j�0Y�D�8����W;ܲ�pTt��6P,� Nǵ��Æ�:(���&�N�/ X��i%�?�_P	�n�F�.^�G�E���鬫>?���"@v�2���A~�aԹ_[P, n��N������_rƢ��    IEND�B`�       ECFG      application/config/name         apoxel     application/run/main_scene         res://main.tscn -   application/run/low_processor_mode_sleep_usec             application/config/icon         res://icon.png  
   input/jump�              deadzone      ?      events              InputEventKey         resource_local_to_scene           resource_name             device            alt           shift             control           meta          command           pressed           scancode          physical_scancode             unicode           echo          script         input/move_forward�              deadzone      ?      events              InputEventKey         resource_local_to_scene           resource_name             device            alt           shift             control           meta          command           pressed           scancode   W      physical_scancode             unicode           echo          script         input/move_backwards�              deadzone      ?      events              InputEventKey         resource_local_to_scene           resource_name             device            alt           shift             control           meta          command           pressed           scancode   S      physical_scancode             unicode           echo          script         input/move_left�              deadzone      ?      events              InputEventKey         resource_local_to_scene           resource_name             device            alt           shift             control           meta          command           pressed           scancode   A      physical_scancode             unicode           echo          script         input/move_right�              deadzone      ?      events              InputEventKey         resource_local_to_scene           resource_name             device            alt           shift             control           meta          command           pressed           scancode   D      physical_scancode             unicode           echo          script         input/window_click�              deadzone      ?      events              InputEventMouseButton         resource_local_to_scene           resource_name             device     ����   alt           shift             control           meta          command           button_mask           position              global_position               factor       �?   button_index         pressed           doubleclick           script      )   physics/common/enable_pause_aware_picking            rendering/threads/thread_model         )   rendering/environment/default_environment          res://default_env.tres                