# [level 3] 카카오 앱 정리하기 - 468374 

[문제 링크](https://school.programmers.co.kr/learn/courses/30/lessons/468374) 

### 성능 요약

메모리: 9.41 MB, 시간: 343.97 ms

### 구분

코딩테스트 연습 > 2025 카카오 하반기 1차

### 채점결과

정확성: 100.0<br/>합계: 100.0 / 100.0

### 제출 일자

2026년 05월 11일 23:19:36

### 문제 설명

<p><code>N</code> x <code>M</code> 크기의 격자판 위에 카카오 앱들이 존재합니다. 격자에서 가장 왼쪽 위칸은 1행 1열, 가장 오른쪽 아래칸은 <code>N</code>행 <code>M</code>열입니다. 모든 앱들은 정사각형 모양이며 크기는 제각각 다를 수 있습니다.</p>

<p>예를 들어, <code>N</code> = 6, <code>M</code> = 8일 때 아래 그림과 같이 앱들이 배치되어 있습니다.</p>

<p><img src="https://grepp-programmers.s3.ap-northeast-2.amazonaws.com/production/presigned_urls/89808f9e-45f4-4a7c-8a6b-ca04b1feed91/kakao_app_00.jpg" title="" alt="kakao_app_00.jpg"></p>

<ul>
<li>6개의 앱이 격자판 위에 존재합니다.</li>
</ul>

<p>당신은 해당 앱들 중 하나를 골라 상하좌우 중 한 방향으로 한 칸 밀 수 있습니다. 이때, 해당 방향에 다른 앱이 존재한다면 해당 앱도 같은 방향으로 한 칸 밀려납니다. 위 예시에서 카카오톡 앱을 오른쪽으로 한 칸 밀면 아래와 같이 됩니다.</p>

<p><img src="https://grepp-programmers.s3.ap-northeast-2.amazonaws.com/production/presigned_urls/acfd35fe-38d9-4287-bca1-da9662e5a942/kakao_app_01.jpg" title="" alt="kakao_app_01.jpg"></p>

<ul>
<li>빨간색으로 표시된 카카오톡 앱을 오른쪽으로 한 칸 밉니다.</li>
<li>초록색으로 표시된 4개의 앱들은 카카오톡에 의해 영향을 받은 앱들을 나타냅니다.</li>
</ul>

<p>만약 앱이 격자 밖으로 이동하게 된다면, 해당 앱은 격자 반대편으로 이어져 나오게 됩니다. 이때, 크기가 2x2 이상인 앱은 한 칸이라도 격자 밖으로 이동하게 되면 반대편으로 이동하게 됩니다. 격자 반대편으로 이동할 때 해당 위치에 다른 앱이 있다면 그 앱도 같은 방향으로 밀려납니다. 위 예시에서 카카오톡 앱을 오른쪽으로 한 칸 밀면 아래와 같이 됩니다.</p>

<p><img src="https://grepp-programmers.s3.ap-northeast-2.amazonaws.com/production/presigned_urls/c7cfe11c-eb92-4c78-977b-396e9ec32605/kakao_app_02.jpg" title="" alt="kakao_app_02.jpg"></p>

<ul>
<li>카카오 뱅크앱이 격자 밖으로 이동하게 되어 격자 반대편으로 이동하게 됩니다.</li>
<li>카카오 웹툰앱(w)의 일부가 격자 밖으로 이동하게 되어 격자 반대편으로 이동하게 됩니다. 이 과정에서 카카오 뮤직 앱이 오른쪽으로 밀리게 됩니다. </li>
</ul>

<p>격자와 앱의 위치 정보를 담은 2차원 정수 배열 <code>board</code>와 앱의 이동 명령을 순서대로 담은 2차원 정수 배열 <code>commands</code>가 매개변수로 주어집니다. 주어진 규칙에 따라 앱을 이동시킨 후 앱의 위치 정보를 2차원 정수 배열에 담아 return 하도록 solution 함수를 완성해 주세요.</p>

<hr>

<h5>제한사항</h5>

<ul>
<li>2 ≤ <code>board</code>의 길이 = <code>N</code> ≤ 10

<ul>
<li>2 ≤ <code>board[i]</code>의 길이 = <code>M</code> ≤ 10</li>
<li>0 ≤ <code>board[i][j]</code> ≤ 100</li>
<li><code>board[i][j]</code>는 앱의 유일한 ID를 나타내며, <code>board</code>의 <code>i+1</code>번째 행 <code>j+1</code>번째 열에 ID가 <code>board[i][j]</code>인 앱이 존재함을 나타냅니다. 각 앱의 ID는 1번부터 <code>앱의 가짓수</code>번까지 번호가 붙어 있습니다.</li>
<li><code>board[i][j]</code>가 같은 원소면 정사각형 모양으로 붙어 있습니다.</li>
<li><code>board[i][j]</code>가 0이면 해당 격자는 빈 칸임을 나타냅니다.</li>
</ul></li>
<li>1 ≤ <code>commands</code>의 길이 ≤ 1,000

<ul>
<li><code>commands[i]</code>는 [<code>ID</code>, <code>arrow</code>]형태로 ID가 <code>ID</code>인 앱을 <code>arrow</code>방향으로 한 칸 움직임을 나타냅니다.</li>
<li>1 ≤ <code>ID</code> ≤ <code>board[i][j]</code>의 최댓값</li>
<li>1 ≤ <code>arrow</code> ≤ 4</li>
<li>1은 오른쪽, 2는 아래쪽, 3은 왼쪽, 4는 위쪽 방향을 나타냅니다.</li>
</ul></li>
</ul>

<hr>

<h5>테스트 케이스 구성 안내</h5>

<p>아래는 테스트 케이스 구성을 나타냅니다. 각 그룹은 하나 이상의 하위 그룹으로 이루어져 있으며, 하위 그룹의 모든 테스트 케이스를 통과하면 해당 그룹에 할당된 점수를 획득할 수 있습니다.</p>
<table class="table">
        <thead><tr>
<th>그룹</th>
<th>총점</th>
<th>테스트 케이스 그룹 설명</th>
</tr>
</thead>
        <tbody><tr>
<td>#1</td>
<td>5%</td>
<td>1x1 크기의 앱 1개만 존재합니다.</td>
</tr>
<tr>
<td>#2</td>
<td>10%</td>
<td>2x2 크기의 앱 1개만 존재합니다.</td>
</tr>
<tr>
<td>#3</td>
<td>15%</td>
<td>모든 앱의 크기가 1x1입니다.</td>
</tr>
<tr>
<td>#4</td>
<td>20%</td>
<td>앱이 격자 밖으로 이동하는 명령이 주어지지 않습니다.</td>
</tr>
<tr>
<td>#5</td>
<td>50%</td>
<td>추가 제한 사항 없음</td>
</tr>
</tbody>
      </table>
<hr>

<h5>입출력 예</h5>
<table class="table">
        <thead><tr>
<th>board</th>
<th>commands</th>
<th>result</th>
</tr>
</thead>
        <tbody><tr>
<td>[[0, 2, 2, 0, 0, 0, 0, 0], [0, 2, 2, 0, 0, 4, 4, 0], [0, 3, 3, 3, 1, 4, 4, 0], [0, 3, 3, 3, 0, 0, 0, 0], [0, 3, 3, 3, 5, 5, 6, 0], [0, 0, 0, 0, 5, 5, 0, 0]]</td>
<td>[[3, 1], [3, 1]]</td>
<td>[[0, 0, 2, 2, 0, 0, 0, 0], [4, 4, 2, 2, 0, 0, 0, 0], [4, 4, 0, 3, 3, 3, 1, 0], [0, 0, 0, 3, 3, 3, 0, 0], [6, 0, 0, 3, 3, 3, 5, 5], [0, 0, 0, 0, 0, 0, 5, 5]]</td>
</tr>
<tr>
<td>[[0, 9, 1, 1, 6, 0, 0, 0], [2, 2, 1, 1, 0, 0, 0, 0], [2, 2, 3, 4, 4, 4, 0, 0], [5, 0, 0, 4, 4, 4, 7, 0], [0, 0, 0, 4, 4, 4, 8, 8], [0, 0, 0, 0, 0, 0, 8, 8]]</td>
<td>[[2, 1], [3, 1], [9, 2], [4, 1]]</td>
<td>[[8, 8, 0, 1, 1, 6, 0, 0], [8, 8, 0, 1, 1, 0, 0, 0], [4, 4, 4, 9, 3, 0, 0, 0], [4, 4, 4, 7, 2, 2, 0, 0], [4, 4, 4, 0, 2, 2, 0, 0], [0, 5, 0, 0, 0, 0, 0, 0]]</td>
</tr>
<tr>
<td>[[1, 1, 0], [1, 1, 0]]</td>
<td>[[1, 4], [1, 3], [1, 2]]</td>
<td>[[0, 1, 1], [0, 1, 1]]</td>
</tr>
</tbody>
      </table>
<h5>입출력 예 설명</h5>

<p><strong>입출력 예 #1</strong></p>

<ul>
<li>문제 예시와 같습니다.</li>
</ul>

<p><strong>입출력 예 #2</strong></p>

<p>아래 그림과 같이 앱들이 배치되어 있습니다.</p>

<p><img src="https://grepp-programmers.s3.ap-northeast-2.amazonaws.com/production/presigned_urls/b87c343d-5041-410b-ac9b-f29c9cead08e/kakao_app_2_0.jpg" title="" alt="kakao_app_2_0.jpg"></p>

<p>2번 앱(카카오 뮤직)을 오른쪽으로 한 칸 움직이면 아래와 같이 됩니다.</p>

<p><img src="https://grepp-programmers.s3.ap-northeast-2.amazonaws.com/production/presigned_urls/aacb94b3-d90e-469d-afb6-cc9332434a55/kakao_app_2_1.jpg" title="" alt="kakao_app_2_1.jpg"></p>

<p>3번 앱(카카오 페이)을 오른쪽으로 한 칸 움직이면 아래와 같이 됩니다.</p>

<p><img src="https://grepp-programmers.s3.ap-northeast-2.amazonaws.com/production/presigned_urls/ce00e0d3-5987-4579-9c70-54e4eeba8d40/kakao_app_2_2.jpg" title="" alt="kakao_app_2_2.jpg"></p>

<p>9번 앱(카카오톡)을 아래쪽으로 한 칸 움직이면 아래와 같이 됩니다. 카카오 웹툰 앱이 위쪽으로 이동하게 되어 카카오톡 앱이 아래로 한 칸 더 밀립니다.</p>

<p><img src="https://grepp-programmers.s3.ap-northeast-2.amazonaws.com/production/presigned_urls/a8beb83c-6bf5-4e24-a450-5ed21d535fad/kakao_app_2_3.jpg" title="" alt="kakao_app_2_3.jpg"></p>

<p>4번 앱(카카오 페이지)을 오른쪽으로 한 칸 움직이면 아래와 같이 됩니다.</p>

<p><img src="https://grepp-programmers.s3.ap-northeast-2.amazonaws.com/production/presigned_urls/71ab123a-613d-4f36-95dd-33766cb63f7c/kakao_app_2_4.jpg" title="" alt="kakao_app_2_4.jpg"></p>

<p><strong>입출력 예 #3</strong></p>

<p>아래 그림과 같이 앱이 배치되어 있습니다.</p>

<p><img src="https://grepp-programmers.s3.ap-northeast-2.amazonaws.com/production/presigned_urls/37393a7e-78cc-4bd1-bf2a-0a2ac732ced7/kakao_app_3_0.jpg" title="" alt="kakao_app_3_0.jpg"></p>

<p>1번 앱(프로그래머스)을 위쪽으로 한 칸 움직이면 아래와 같이 됩니다.</p>

<p><img src="https://grepp-programmers.s3.ap-northeast-2.amazonaws.com/production/presigned_urls/ba46512f-177e-4721-9cb5-ad18cf80ee34/kakao_app_3_1.jpg" title="" alt="kakao_app_3_1.jpg"></p>

<p>1번 앱(프로그래머스)을 왼쪽으로 한 칸 움직이면 아래와 같이 됩니다.</p>

<p><img src="https://grepp-programmers.s3.ap-northeast-2.amazonaws.com/production/presigned_urls/ea5787bc-594a-4da7-b17c-7fdb778ea2d4/kakao_app_3_2.jpg" title="" alt="kakao_app_3_2.jpg"></p>

<p>1번 앱(프로그래머스)을 아래쪽으로 한 칸 움직이면 아래와 같이 됩니다.</p>

<p><img src="https://grepp-programmers.s3.ap-northeast-2.amazonaws.com/production/presigned_urls/79427532-2a82-405f-b0be-b106d8ab8ec3/kakao_app_3_3.jpg" title="" alt="kakao_app_3_3.jpg"></p>


> 출처: 프로그래머스 코딩 테스트 연습, https://school.programmers.co.kr/learn/challenges