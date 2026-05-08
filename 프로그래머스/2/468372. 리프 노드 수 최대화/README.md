# [level 2] 리프 노드 수 최대화 - 468372 

[문제 링크](https://school.programmers.co.kr/learn/courses/30/lessons/468372) 

### 성능 요약

메모리: 8.76 MB, 시간: 8401.11 ms

### 구분

코딩테스트 연습 > 2025 카카오 하반기 1차

### 채점결과

정확성: 100.0<br/>합계: 100.0 / 100.0

### 제출 일자

2026년 05월 08일 18:46:34

### 문제 설명

<p>루트 노드, 리프 노드, 분배 노드로 구성되는 트리가 있습니다. 루트 노드는 자식 노드를 하나만 가지며, 루트 노드가 아닌 노드는 자식 노드를 2개, 3개 또는 0개를 가질 수 있습니다. 자식 노드가 0개인 노드는 리프 노드입니다. 당신은 제한된 조건 하에서 트리를 하나 구성하여 리프 노드를 가능한 한 많이 만들려고 합니다.</p>

<p>트리를 구성하는 규칙은 아래와 같습니다.</p>

<ol>
<li>루트 노드와 리프 노드를 제외한 나머지 노드를 분배 노드라고 하며, 분배 노드는 자식 노드를 2개 또는 3개를 갖습니다.</li>
<li>분배 노드는 최대 <code>dist_limit</code>개 존재할 수 있습니다.</li>
<li>트리에서 같은 깊이에 있는 분배 노드의 자식 노드 수는 모두 같아야 합니다. 노드의 깊이는 루트 노드부터 해당 노드까지의 최단 경로 길이와 같습니다.</li>
<li>모든 리프 노드는 분배도라는 값을 갖습니다. 분배도는 해당 리프 노드의 부모 노드에서 루트 노드까지의 최단 경로 상에 있는 모든 노드의 자식 노드 개수의 곱과 같습니다. 예를 들어 아래 그림과 같이 트리가 주어진 경우, 1번 리프 노드의 분배도는 3 × 1 = 3이며, 2번 리프 노드의 분배도는 2 × 3 × 3 × 1 = 18입니다.</li>
<li>모든 리프 노드의 분배도는 <code>split_limit</code>보다 작거나 같아야 합니다.</li>
</ol>

<p><img src="https://grepp-programmers.s3.ap-northeast-2.amazonaws.com/production/presigned_urls/9f068cda-13dc-4a0a-982a-92bd69912604/split_score.png" title="" alt="split_score.png"></p>

<p>예를 들어 <code>dist_limit</code>가 3이고 <code>split_limit</code>가 6인 경우, 아래와 같이 트리를 구성할 수 있습니다.</p>

<p><img src="https://grepp-programmers.s3.ap-northeast-2.amazonaws.com/production/presigned_urls/0c0df805-d9cc-40f5-b8ab-d9b8d45c16dd/convery_ex0.png" title="" alt="convery_ex0.png"></p>

<p>위와 같이 구성하면 리프 노드의 수는 6이 되며, 분배 노드의 수는 3, 모든 리프 노드의 분배도는 3 × 2 × 1 = 6이므로 조건을 만족합니다. 주어진 조건 하에 리프 노드의 수를 6개보다 많이 만드는 트리 구성은 존재하지 않습니다.</p>

<p>최대 몇 개의 분배 노드를 놓을 수 있는지 나타내는 정수 <code>dist_limit</code>, 분배도의 최댓값을 나타내는 정수 <code>split_limit</code>가 매개변수로 주어집니다. 주어진 조건 하에서 만들 수 있는 트리의 리프 노드 수의 최댓값을 return 하도록 solution 함수를 완성해 주세요.</p>

<hr>

<h5>제한사항</h5>

<ul>
<li>0 ≤ <code>dist_limit</code> ≤ 10<sup>9</sup></li>
<li>1 ≤ <code>split_limit</code> ≤ 10<sup>9</sup></li>
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
<td>30%</td>
<td><code>dist_limit ≤ 10</code>, <code>split_limit ≤ 50</code></td>
</tr>
<tr>
<td>#2</td>
<td>70%</td>
<td>추가 제한 사항 없음</td>
</tr>
</tbody>
      </table>
<hr>

<h5>입출력 예</h5>
<table class="table">
        <thead><tr>
<th>dist_limit</th>
<th>split_limit</th>
<th>result</th>
</tr>
</thead>
        <tbody><tr>
<td>3</td>
<td>6</td>
<td>6</td>
</tr>
<tr>
<td>0</td>
<td>10</td>
<td>1</td>
</tr>
<tr>
<td>3</td>
<td>100</td>
<td>7</td>
</tr>
<tr>
<td>5</td>
<td>16</td>
<td>9</td>
</tr>
</tbody>
      </table>
<hr>

<h5>입출력 예 설명</h5>

<p><strong>입출력 예 #1</strong></p>

<p>문제 예시와 같습니다.</p>

<p><strong>입출력 예 #2</strong></p>

<p>아래 그림과 같이 트리를 구성할 수 있습니다.</p>

<p><img src="https://grepp-programmers.s3.ap-northeast-2.amazonaws.com/production/presigned_urls/277c28e5-04d9-4fe5-a77e-1344671b6552/convery_ex2.png" title="" alt="convery_ex2.png"></p>

<p>분배 노드를 놓을 수 없으므로 루트 노드의 자식 노드 1개가 트리에서 유일한 리프 노드입니다. 따라서 1을 return 해야 합니다.</p>

<p><strong>입출력 예 #3</strong></p>

<p>아래 그림과 같이 트리를 구성할 수 있습니다.</p>

<p><img src="https://grepp-programmers.s3.ap-northeast-2.amazonaws.com/production/presigned_urls/f7d47634-f0d3-4569-9b13-277530a93814/convey_ex3.png" title="" alt="convey_ex3.png"></p>

<p>다른 트리 구성으로도 리프 노드를 7개 만들 수 있지만 7개보다 많은 리프 노드를 만들 수 있는 트리 구성은 존재하지 않습니다. 따라서 7을 return 해야 합니다.</p>

<p><strong>입출력 예 #4</strong></p>

<p>아래 그림과 같이 트리를 구성할 수 있습니다.</p>

<p><img src="https://grepp-programmers.s3.ap-northeast-2.amazonaws.com/production/presigned_urls/d8fdf0ee-3520-46a1-be8d-7e7956d59052/convey_ex4.png" title="" alt="convey_ex4.png"></p>

<p>분배 노드는 최대 5개까지 놓을 수 있지만, 위와 같이 4개까지만 놓는 것도 가능합니다. 9개보다 많은 리프 노드를 만들 수 있는 다른 트리 구성은 존재하지 않습니다. 따라서 9를 return 해야 합니다.</p>


> 출처: 프로그래머스 코딩 테스트 연습, https://school.programmers.co.kr/learn/challenges