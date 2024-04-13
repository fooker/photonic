

function update(ctx, out)
    size = out.size
    out:update(function (i, _)
        return i/size, 0, (size-i)/size
    end)

    for i = 0, out.size - 1, 7 do
        out:set(i, 0, 1, 0)
    end
end