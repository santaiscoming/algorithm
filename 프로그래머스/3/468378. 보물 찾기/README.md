# [level 3] 보물 찾기 - 468378 

[문제 링크](https://school.programmers.co.kr/learn/courses/30/lessons/468378) 

### 성능 요약

메모리: 18.6 MB, 시간: 725.29 ms

### 구분

코딩테스트 연습 > 2025 카카오 하반기 2차

### 채점결과

정확성: 100.0<br/>합계: 100.0 / 100.0

### 제출 일자

2026년 05월 23일 20:39:30

### 문제 설명

<p><code>w</code> × <code>h</code> 크기의 직사각형 격자 모양의 땅 속에 보물이 있습니다. 보물은 단 하나 존재하며, 격자 칸 중 한 곳에 있습니다. 당신은 굴착 로봇으로 보물을 찾아내 발굴하려 합니다.</p>

<p>굴착 로봇에 명령을 보내면 격자에서 원하는 열을 세로로 파낼 수 있습니다. </p>

<p>격자의 각 열은 굴착 가능한 최대 깊이가 각각 다를 수 있습니다. 굴착 로봇이 <code>col</code>열을 파도록 명령을 보내면 가능한 최대 깊이만큼 땅을 판 뒤 돌아옵니다. 만약 <code>col</code>열에 보물이 있었다면 보물을 가지고 돌아오며, 보물이 없었더라도 <code>col</code>열을 기준으로 왼쪽 방향에 보물이 있는지 오른쪽 방향에 보물이 있는지에 대한 정보를 가지고 돌아옵니다. 이때 땅을 팔 때마다 굴착 가능한 깊이만큼 비용이 발생합니다.</p>

<p>각 열의 최대 깊이를 담은 1차원 정수 배열 <code>depth</code>와 사용 가능한 총비용을 나타내는 정수 <code>money</code>가 매개변수로 주어집니다. 또한 굴착 로봇이 특정 열을 파도록 하는 <code>excavate</code> 함수가 주어집니다. </p>

<p>이때, <code>excavate</code> 함수를 호출해 보물을 찾아 발굴하고, 보물이 있었던 열을 return 하도록 solution 함수를 완성해 주세요. 정답 판정을 받으려면 총비용이 <code>money</code>를 초과하지 않아야 하며, <code>excavate</code> 함수를 호출해 한 번 이상 0을 return 받고, 보물이 있었던 열을 return 해야 합니다. <code>money</code>보다 적은 비용으로 보물을 찾더라도 추가 점수가 주어지지는 않습니다. 따라서 각 채점 테스트 케이스에서 총비용이 <code>money</code>를 초과하지 않고 보물을 찾아냈다면, 사용한 총비용과 관계없이 동일한 점수를 받습니다.</p>

<hr>

<h5>제한사항</h5>

<ul>
<li>2 ≤ <code>depth</code>의 길이 = <code>w</code> ≤ 200

<ul>
<li><code>depth[i]</code>는 <code>i+1</code>열의 굴착 가능한 최대 깊이를 나타냅니다.</li>
<li>1 ≤ <code>depth[i]</code> ≤ 100,000</li>
</ul></li>
<li>1 ≤ <code>money</code> ≤ <code>depth</code>의 원소의 합

<ul>
<li>운에 맡기지 않고 100% 확률로 보물을 찾기 위한 최소 비용 ≤ <code>money</code></li>
</ul></li>
<li><code>excavate</code> 함수는 굴착할 열의 위치(1 이상 <code>w</code> 이하의 정수)를 전달받고, 굴착 결과에 따라 -1, 0, 1 중 하나를 return 합니다. 

<ul>
<li>1 ~ <code>w</code> 사이의 정수가 아닌 값을 전달하는 경우 오답으로 판정합니다.</li>
<li>보물을 찾은 경우 0, 보물이 왼쪽 방향에 있다면 -1, 오른쪽 방향에 있다면 1을 return 합니다. </li>
<li><code>excavate</code> 함수를 호출할 때마다 굴착할 열의 최대 깊이만큼 비용이 발생합니다. </li>
<li>주어진 비용을 초과하지 않는 범위 내에서 <code>excavate</code> 함수를 원하는 만큼 호출해도 됩니다.</li>
<li><code>excavate</code> 함수 사용 예시가 초기 코드로 주어집니다. 해당 코드는 1열 ~ <code>w</code>열을 순서대로 굴착해, 보물을 찾은 경우 해당 열을 return 하는 코드입니다.</li>
</ul></li>
</ul>

<hr>

<h5>입출력 예</h5>
<table class="table">
        <thead><tr>
<th>depth</th>
<th>money</th>
<th>result</th>
</tr>
</thead>
        <tbody><tr>
<td>[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]</td>
<td>55</td>
<td>3</td>
</tr>
<tr>
<td>[1, 1, 1, 1, 1]</td>
<td>3</td>
<td>5</td>
</tr>
<tr>
<td>[2, 100, 1, 100, 3, 100, 1]</td>
<td>200</td>
<td>6</td>
</tr>
<tr>
<td>[2, 100, 1, 100, 3, 100, 1]</td>
<td>200</td>
<td>5</td>
</tr>
<tr>
<td>[3, 2, 1, 2, 3, 2, 1, 2]</td>
<td>8</td>
<td>5</td>
</tr>
<tr>
<td>[1, 1000, 1, 1, 1, 10, 15, 1]</td>
<td>1002</td>
<td>2</td>
</tr>
</tbody>
      </table>
<ul>
<li>예시로 주어지는 테스트 케이스는 보물의 위치가 고정되어 있습니다.</li>
</ul>

<hr>

<h5>채점 테스트 케이스</h5>

<ul>
<li>정답 판정을 받은 채점 테스트 케이스에 따라, 정해진 점수를 획득할 수 있습니다.

<ul>
<li>채점 테스트케이스 일부는 보물의 위치가 고정되어 있습니다.</li>
<li>채점 테스트케이스 일부는 solution 함수의 굴착 전략에 맞추어 <code>excavate</code>를 호출할 때마다 매번 보물의 위치가 바뀔 수 있습니다. (단, 이전 호출과 보물의 위치가 모순되는 경우는 없습니다.)</li>
<li>채점 테스트케이스 일부는 <code>depth</code>의 모든 원소가 똑같은 값입니다.</li>
</ul></li>
</ul>

<hr>

<h5>입출력 예 설명</h5>

<p><strong>입출력 예 #1</strong></p>

<p>보물은 3열에 있습니다. </p>

<ul>
<li>excavate 함수에 3보다 큰 정수를 전달하면 -1이 반환됩니다.</li>
<li>excavate 함수에 3을 전달하면 0이 반환됩니다.</li>
<li>excavate 함수에 3보다 작은 정수를 전달하면 1이 반환됩니다.</li>
</ul>

<p>excavate 함수에 3을 전달해 0을 반환받고, 총비용이 55를 초과하지 않고, 3을 return 하면 정답 판정을 받습니다.</p>

<p><strong>입출력 예 #2</strong></p>

<p>보물은 5열에 있습니다. excavate 함수에 5를 전달해 0을 반환받고, 총비용이 3을 초과하지 않고, 5를 return 하면 정답 판정을 받습니다.</p>

<p><strong>입출력 예 #3</strong></p>

<p>보물은 6열에 있습니다. excavate 함수에 6을 전달해 0을 반환받고, 총비용이 200을 초과하지 않고, 6을 return 하면 정답 판정을 받습니다.</p>

<p><strong>입출력 예 #4</strong></p>

<p>보물은 5열에 있습니다. excavate 함수에 5를 전달해 0을 반환받고, 총비용이 200을 초과하지 않고, 5를 return 하면 정답 판정을 받습니다.</p>

<p><strong>입출력 예 #5</strong></p>

<p>보물은 5열에 있습니다. excavate 함수에 5를 전달해 0을 반환받고, 총비용이 8을 초과하지 않고, 5를 return 하면 정답 판정을 받습니다.</p>

<p><strong>입출력 예 #6</strong></p>

<p>보물은 2열에 있습니다. excavate 함수에 2를 전달해 0을 반환받고, 총비용이 1002를 초과하지 않고, 2를 return 하면 정답 판정을 받습니다.</p>


> 출처: 프로그래머스 코딩 테스트 연습, https://school.programmers.co.kr/learn/challenges