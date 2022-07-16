const $board = $('#board');
const ROWS = 10;
const COLS = 10;

function createBoard(rows, cols)
{
    $board.empty();
    let counter = 1;
    for(let i=0;i<rows;i++)
    {
        const $row = $('<div>').addClass('row');
        
        for(let j=0;j<cols;j++)
        {
                const $col = $('<div>').addClass('col hidden').attr('data-col', j).attr('data-row',i).attr('id', counter);
                $row.append($col);
                counter++;
        }
                
        $board.append($row);
    }
    const bombList=[];
    const bombNumber = Math.floor(rows*cols/10);
    while(bombList.length<bombNumber)
    {
        let ranNum = Math.floor(Math.random() * rows*cols)+1;
        if(bombList.indexOf(ranNum)===-1)
        {
            bombList.push(ranNum);
        }
    }
    bombList.sort((a,b)=> a-b );
    for(let k in bombList)
    {
        $("#"+bombList[k]+"").addClass('bomb');
    }
    $('.col.hidden').each(function()
    {
        if(!($(this).hasClass('bomb')))
        {
            const row = parseInt($(this).data('row'));
            const col = parseInt($(this).data('col'));
            let count = getBombCount(row, col);
            $(this).html(count);
            
        }
        
    });
    
    
}

createBoard(ROWS, COLS);

function getBombCount(i,j)
{
    let count = 0;
    for(let di=-1; di<=1; di++)
    {
        for(let dj = -1;dj<=1;dj++)
        {
            const ni = i + di;
            const nj = j + dj;
            if(ni >= ROWS || nj >= COLS || nj<0 || ni<0) continue;
            const $cell = $('.col.hidden[data-row='+ni+'][data-col='+nj+']');
            if($cell.hasClass('bomb')){count++;}
            
        }
    }
    if(count==0)
    {
        return "";
    }
    else
    {
        return count;
    }
}

$board.on('mousedown', '.col.hidden', function(event)
{
    switch(event.which)
    {
        case 1:
            if(!($(this).hasClass('flagged')))
            {
                if($(this).hasClass('bomb') )
                {
                    //her afsluttes spillet
                    $(this).removeClass('hidden');
                    $('.bomb').removeClass('hidden');
                    $(this).addClass('activebomb');
                    setTimeout(function(){
                        alert('Gameover');
                        createBoard(ROWS, COLS);
                    }, 350);
                }
                else
                {
                    const row = parseInt($(this).data('row'));
                    const col = parseInt($(this).data('col'));
                    reveal(row, col);
                }
            }
            break;
        case 2:
            console.log("Midtclick");
            break;
        case 3:
            $(this).toggleClass('flagged');
            break;
        default:
    }
});


$(function()
{
    $board.bind("contextmenu", function(e)
    {
            e.preventDefault();
    });
        
});

function reveal(oi, oj)
{
    function helper(i, j)
    {
        if(i >= ROWS || j >= COLS || i<0 || j<0)
        {return;}

        const $cell = $('.col.hidden[data-row='+i+'][data-col='+j+']');
            if(!$cell.hasClass('hidden') || $cell.hasClass('bomb')){
                return;
            }
        $cell.removeClass('hidden');
        
        if($cell.html()!=''){ return;}
        for (let di =-1; di<=1; di++){
            for(let dj=-1;dj <=1; dj++){
                helper(i+di, j+dj);
            }
        }
    }
    helper(oi, oj);
    if ($(".col.hidden").length === $(".col.bomb").length) 
    setTimeout(function(){
        alert("You win");
        createBoard(ROWS, COLS);
    }, 500);
}