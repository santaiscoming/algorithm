# [level 3] 발전소 회로 복구 - 468375 

[문제 링크](https://school.programmers.co.kr/learn/courses/30/lessons/468375) 

### 성능 요약

메모리: 20.7 MB, 시간: 673.49 ms

### 구분

코딩테스트 연습 > 2025 카카오 하반기 1차

### 채점결과

정확성: 100.0<br/>합계: 100.0 / 100.0

### 제출 일자

2026년 05월 16일 21:42:35

### 문제 설명

<p>대정전이 발생한 발전소는 1~<code>h</code>층으로 이루어진 <code>h</code>층 건물이며, 각 층은 같은 구조의 <code>n</code>×<code>m</code> 크기의 격자로 이루어져 있습니다. 각 칸은 통행 가능한 통로('.'), 통행 불가능한 폐쇄 구역('#'), 엘리베이터('@') 중 하나입니다. 엘리베이터는 격자에서 단 한 칸에만 존재하며 다른 층으로 이동하려면 반드시 엘리베이터를 이용해야 합니다. </p>

<p>통로 중 <code>k</code>곳에는 1~<code>k</code>번의 서로 다른 번호를 가진 회로 패널이 1개씩 설치되어 있습니다. 패널의 위치는 [<code>f</code>, <code>r</code>, <code>c</code>] 형태로 주어지며, <code>f</code>층의 <code>r</code>행 <code>c</code>열에 해당 패널이 위치함을 나타냅니다. 초기에 모든 패널은 비활성화 상태이며, <strong>기술자는 항상 1번 패널이 설치된 위치에서 출발해 모든 회로 패널을 활성화해야 합니다.</strong></p>

<p>기술자는 상하좌우로 인접한 칸으로 1초에 1칸씩 이동할 수 있습니다. 엘리베이터 칸에서만 다른 층으로 이동할 수 있으며, 한 층을 이동할 때마다 1초가 필요합니다. 즉 <code>x</code>층에서 <code>y</code>층으로 이동할 때 <code>|x-y|</code> 만큼 시간이 소요됩니다. 엘리베이터를 기다리는 시간이나 타고 내리는 시간은 고려하지 않습니다.</p>

<p>각 패널은 안전 순서를 따라 활성화해야 합니다. 안전 순서는 여러 개의 [<code>a</code>, <code>b</code>] 쌍으로 이루어져 있으며, <code>b</code>번 패널을 활성화하기 전에 <code>a</code>번 패널이 이미 활성화되어 있어야 함을 나타냅니다.</p>

<p>기술자가 패널이 설치된 칸에 위치할 때 그 패널을 즉시 활성화할 수 있습니다. 단, 안전 순서가 아직 충족되지 않은 패널은 활성화할 수 없습니다. 기술자가 <code>x</code>번 패널을 활성화하기 위해서는 안전 순서에 존재하는 <code>b</code> = <code>x</code>인 모든 [<code>a</code>, <code>b</code>] 쌍에 대해서 모든 <code>a</code>번 패널이 이미 활성화되어 있어야 합니다.</p>

<ul>
<li>예를 들어, 안전 순서 쌍이 [[3, 1], [4, 1], [5, 1], [2, 3]]이라고 가정해 보겠습니다.

<ul>
<li>1번 패널을 활성화하려면 3, 4, 5번 세 패널이 이미 활성화되어 있어야 합니다.</li>
<li>3번 패널을 활성화하려면 2번 패널이 이미 활성화되어 있어야 합니다.</li>
<li>2, 4, 5번 패널은 선행 제약 없이 바로 활성화 가능합니다.</li>
</ul></li>
</ul>

<p>당신은 기술자가 모든 패널을 안전하게 활성화하는 데 필요한 최소 시간을 구해야 합니다.</p>

<p>예를 들어 3층인 발전소 내부 구조가 아래 그림과 같다고 가정해 보겠습니다. 엘리베이터가 5행 2열에 위치해 있습니다.</p>

<p><img src="https://grepp-programmers.s3.ap-northeast-2.amazonaws.com/production/presigned_urls/36e96f6a-6009-42d4-a865-82d41387368b/ex1.png" title="" alt="ex1.png"></p>

<p><strong>패널의 위치</strong></p>

<ul>
<li>1번 패널 : 2층 3행 4열</li>
<li>2번 패널 : 2층 5행 6열</li>
<li>3번 패널 : 1층 1행 1열</li>
<li>4번 패널 : 3층 6행 3열</li>
</ul>

<p><strong>안전 순서</strong> : [[3, 2], [1, 2], [4, 1], [4, 3]]</p>

<ul>
<li>1번 패널을 활성화하려면 4번 패널이 이미 활성화되어 있어야 합니다.</li>
<li>2번 패널을 활성화하려면 1, 3번 패널이 이미 활성화되어 있어야 합니다.</li>
<li>3번 패널을 활성화하려면 4번 패널이 이미 활성화되어 있어야 합니다.</li>
<li>4번 패널은 선행 제약 없이 바로 활성화 가능합니다.</li>
</ul>

<p>처음에 기술자가 1번 패널의 위치인 2층 3행 4열에서 출발합니다. 1번 패널은 4번 패널이 활성화되어 있어야 하므로 아직 활성화할 수 없습니다.</p>

<p><img src="https://grepp-programmers.s3.ap-northeast-2.amazonaws.com/production/presigned_urls/c5dace03-3aab-46bd-b31b-f77339937955/ex1-1.png" title="" alt="ex1-1.png"></p>

<ul>
<li>빨간색으로 칠해진 격자는 현재 기술자의 위치를 나타냅니다.</li>
</ul>

<p>엘리베이터로 이동해 2→3층으로 이동합니다. 4번 패널로 이동해 활성화시킵니다. 총 7(= 4 + 1 + 2)초가 걸립니다.</p>

<p><img src="https://grepp-programmers.s3.ap-northeast-2.amazonaws.com/production/presigned_urls/e4cb52ef-aa71-4870-be0a-5262a93472b0/ex1-2.png" title="" alt="ex1-2.png"></p>

<p>엘리베이터로 이동해 3→1층으로 이동합니다. 3번 패널로 이동해 활성화시킵니다. 총 11(= 2 + 2 + 7)초가 걸립니다.</p>

<p><img src="https://grepp-programmers.s3.ap-northeast-2.amazonaws.com/production/presigned_urls/37d442f0-4cec-414d-8d60-4a8852620203/ex1-3.png" title="" alt="ex1-3.png"></p>

<p>엘리베이터로 이동해 1→2층으로 이동합니다. 1번 패널로 이동해 활성화시킵니다. 이어서 2번 패널로 이동해 활성화시킵니다. 총 18(= 7 + 1 + 4 + 6)초가 걸립니다.</p>

<p><img src="https://grepp-programmers.s3.ap-northeast-2.amazonaws.com/production/presigned_urls/5e479f2b-4a82-486a-aa20-f010664e3844/ex1-4.png" title="" alt="ex1-4.png"></p>

<p>위와 같은 방법으로 모든 패널을 안전하게 활성화시킬 수 있으며, 총 시간이 36초가 필요합니다. 이보다 빠른 시간 내로 모든 패널을 안전하게 활성화시킬 수는 없습니다.</p>

<p>발전소의 층 수인 정수 <code>h</code>, 발전소 구조 정보를 담은 1차원 문자열 배열 <code>grid</code>, 패널의 위치를 담은 2차원 정수 배열 <code>panels</code>와 안전 순서를 담은 2차원 정수 배열 <code>seqs</code>가 매개변수로 주어집니다. 기술자가 모든 패널을 안전하게 활성화하는 데 필요한 최소 시간을 return 하도록 solution 함수를 완성해 주세요.</p>

<hr>

<h5>제한사항</h5>

<ul>
<li>1 ≤ <code>h</code> ≤ 10</li>
<li>2 ≤ <code>grid</code>의 길이 = <code>n</code> ≤ 40

<ul>
<li>2 ≤ <code>grid[i]</code>의 원소 길이 = <code>m</code> ≤ 40</li>
<li><code>grid[i][j]</code>는 '.', '#', '@' 중 하나로, <code>i+1</code>행 <code>j+1</code>열 칸의 정보를 나타냅니다. '@'은 격자 내에 반드시 하나만 존재합니다.</li>
</ul></li>
<li>2 ≤ <code>panels</code>의 길이 = <code>k</code> ≤ 15

<ul>
<li><code>panels[i]</code>는 [<code>f</code>, <code>r</code>, <code>c</code>] 형태로 <code>i+1</code>번 패널이 <code>f</code>층의 <code>r</code>행 <code>c</code>열 칸에 위치함을 나타냅니다.</li>
<li>1 ≤ <code>f</code> ≤ <code>h</code></li>
<li>1 ≤ <code>r</code> ≤ <code>n</code></li>
<li>1 ≤ <code>c</code> ≤ <code>m</code></li>
<li><code>grid[r-1][c-1]</code> = '.'</li>
<li><code>panels</code>의 원소는 중복되지 않습니다.</li>
</ul></li>
<li>1 ≤ <code>seqs</code>의 길이 ≤ 100

<ul>
<li><code>seqs</code>의 원소는 [<code>a</code>, <code>b</code>] 형태로 <code>b</code>번 패널을 활성화하기 전에 <code>a</code>번 패널이 이미 활성화되어 있어야 함을 나타냅니다.</li>
<li>1 ≤ <code>a</code> ≤ <code>k</code></li>
<li>1 ≤ <code>b</code> ≤ <code>k</code></li>
<li><code>a</code> ≠ <code>b</code></li>
<li><code>seqs</code>의 원소는 중복되지 않습니다.</li>
</ul></li>
<li>모든 패널을 안전하게 활성화 가능한 경우만 주어집니다.</li>
</ul>

<hr>

<h5>테스트 케이스 구성 안내</h5>

<p>아래는 테스트 케이스 구성을 나타냅니다. 각 그룹은 하나 이상의 하위 그룹으로 이루어져 있으며, 하위 그룹의 모든 테스트 케이스를 통과하면 해당 그룹에 할당된 점수를 획득할 수 있습니다.</p>
<table class="table">
        <thead><tr>
<th>그룹</th>
<th>총점</th>
<th>추가 제한 사항</th>
</tr>
</thead>
        <tbody><tr>
<td>#1</td>
<td>13%</td>
<td><code>k</code> = 2</td>
</tr>
<tr>
<td>#2</td>
<td>18%</td>
<td><code>k</code> ≤ 5</td>
</tr>
<tr>
<td>#3</td>
<td>22%</td>
<td><code>h</code> = 1</td>
</tr>
<tr>
<td>#4</td>
<td>17%</td>
<td><code>seqs</code>의 길이 = <code>k-1</code>, <code>seqs[i]</code> = [<code>i+1</code>, <code>i+2</code>]</td>
</tr>
<tr>
<td>#5</td>
<td>30%</td>
<td>추가 제한 사항 없음</td>
</tr>
</tbody>
      </table>
<hr>

<h5>입출력 예</h5>
<table class="table">
        <thead><tr>
<th>h</th>
<th>grid</th>
<th>panels</th>
<th>seqs</th>
<th>result</th>
</tr>
</thead>
        <tbody><tr>
<td>3</td>
<td>[".#.##..", ".#..##.", ".......", "##.###.", ".@.#...", "...#..."]</td>
<td>[[2, 3, 4], [2, 5, 6], [1, 1, 1], [3, 6, 3]]</td>
<td>[[3, 2], [1, 2], [4, 1], [4, 3]]</td>
<td>36</td>
</tr>
<tr>
<td>1</td>
<td>["@......", ".######", ".......", "######.", ".......", ".####..", "....#.."]</td>
<td>[[1, 7, 4], [1, 3, 5], [1, 1, 3]]</td>
<td>[[1, 3], [3, 2]]</td>
<td>31</td>
</tr>
<tr>
<td>4</td>
<td>["........#", "........#", "@.......#", ".#.#....#", "........#", "#........", "#.#..####", "..#..####", ".....####"]</td>
<td>[[2, 9, 1], [2, 1, 8], [1, 1, 3], [3, 3, 2], [1, 2, 8]]</td>
<td>[[1, 2], [2, 3], [3, 4], [4, 5]]</td>
<td>47</td>
</tr>
</tbody>
      </table>
<hr>

<h5>입출력 예 설명</h5>

<p><strong>입출력 예 #1</strong></p>

<p>문제 예시와 같습니다.</p>

<p><strong>입출력 예 #2</strong></p>

<p>테스트 케이스 그룹 #3의 추가 제한 사항을 만족하는 예시입니다. </p>

<p>1층인 발전소 내부 구조가 아래 그림과 같습니다. 엘리베이터가 1행 1열에 위치해 있습니다. </p>

<p><img src="https://grepp-programmers.s3.ap-northeast-2.amazonaws.com/production/presigned_urls/0687aa7c-8e29-468d-9308-6b0cf6e606bb/ex2.png" title="" alt="ex2.png"> </p>

<p><strong>패널의 위치</strong> </p>

<ul>
<li>1번 패널 : 1층 7행 4열 </li>
<li>2번 패널 : 1층 3행 5열 </li>
<li>3번 패널 : 1층 1행 3열 </li>
</ul>

<p><strong>안전 순서</strong> : [[1, 3], [3, 2]] </p>

<ul>
<li>1번 패널을 선행 제약 없이 바로 활성화 가능합니다. </li>
<li>2번 패널을 활성화하려면 3번 패널이 이미 활성화되어 있어야 합니다. </li>
<li>3번 패널을 활성화하려면 1번 패널이 이미 활성화되어 있어야 합니다. </li>
</ul>

<p>아래 그림처럼 이동하면서 1→3→2번 패널 순서대로 활성화시키면 모든 패널을 안전하게 활성화시킬 수 있으며, 총 시간이 31초가 필요합니다. 이보다 빠른 시간 내로 모든 패널을 안전하게 활성화시킬 수는 없습니다. </p>

<p><img src="https://grepp-programmers.s3.ap-northeast-2.amazonaws.com/production/presigned_urls/ed8baf42-b46e-4ce1-907f-a0e85e29cb99/ex2-1.png" title="" alt="ex2-1.png"></p>

<p>따라서 31을 return 합니다. </p>

<p><strong>입출력 예 #3</strong> </p>

<p>테스트 케이스 그룹 #4의 추가 제한 사항을 만족하는 예시입니다. </p>

<p>1→2→3→4→5번 패널 순서대로 활성화시키면 모든 패널을 안전하게 활성화시킬 수 있으며, 총 시간이 47초가 필요합니다. 이보다 빠른 시간 내로 모든 패널을 안전하게 활성화시킬 수는 없습니다. 따라서 47을 return 합니다.</p>


> 출처: 프로그래머스 코딩 테스트 연습, https://school.programmers.co.kr/learn/challenges